use clap::{Arg, App};
use iz80::*;

mod pds_machine;
#[cfg(unix)]
mod console_unix;

use self::pds_machine::PdsMachine;

// Welcome message
const WELCOME: &str =
"Emulation of the Zilog MCZ-1 computer
https://github.com/ivanizag/izilogpds\n";

static DISK: &[u8] = include_bytes!("../disks/13-1000-01-UNABRIDGED_SYSTEM_DISK.MCZ");
//static DISK: &[u8] = include_bytes!("../disks/13-3001-01_MCZ1-20_RIO_206.MCZ");
//static DISK: &[u8] = include_bytes!("../disks/13-3001-03_MCZ-PDS_RIO_2-2.MCZ");
//static DISK: &[u8] = include_bytes!("../disks/13-3001-03_MCZ-PDS_RIO_220-MCZIMAGER.MCZ");


const RBDIN_SYNC: u8 = 0x0a;
const RBDIN_ASYNC: u8 = 0x0b;
const WRTBIN_SYNC: u8 = 0x0e;
const WRTBIN_ASYNC: u8 = 0x0f;

const SECTOR_SIZE: usize = 128;
const SECTOR_SIZE_IN_FILE: usize = SECTOR_SIZE + 8;
const SECTOR_COUNT: usize = 32;
//const TRACK_COUNT: usize = 77;

fn read_disk(machine: &mut PdsMachine, address: u16, sector: u8, track: u8) {
    let start = (track as usize * SECTOR_COUNT + sector as usize) * SECTOR_SIZE_IN_FILE;

    //print!("READ DISK: address={:04x}h sector={:02} track={:02} offset={:06x}h\n", address, sector, track, start);

    let sector_in_file = DISK[start] & 0x7f;
    if sector != sector_in_file {
        panic!("Sector {} in file has sector {}", sector, sector_in_file);
    }

    let track_in_file = DISK[start + 1];
    if track != track_in_file {
        panic!("Track {} in file has track {}", track, track_in_file);
    }

    for i in 0..SECTOR_SIZE {
        machine.poke(address+i as u16, DISK[start+2+i]);
    }

    // Store the pointers and CRC on the PROM working memory, RIO OS reads those bytes.
    for i in 0..6 {
        machine.poke(0x12b4+i as u16, DISK[start+2+SECTOR_SIZE+i]);
    }
}

fn rom_floopy(machine: &mut PdsMachine, iy: u16, floppy_trace: bool) -> u16 {
    let request = machine.peek(iy+1);
    let mut data_address = machine.peek16(iy+2);
    let mut data_length = machine.peek16(iy+4) as usize;
    let completion_return_address = machine.peek16(iy+6);
    let error_return_address = machine.peek16(iy+8);
    let volume_sector = machine.peek(iy+11);
    let volume = volume_sector >> 5;
    let sector = volume_sector & 0x1f;
    let track = machine.peek(iy+12);

    if floppy_trace {
        print!("Floopy: request={:02x} volume={} track={} sector={} data_address={:04x} data_length={} completion_return_address={:04x} error_return_address={:04x}\n",
            request, volume, track, sector, data_address, data_length, completion_return_address, error_return_address);
    }

    if data_length % SECTOR_SIZE != 0{
        data_length = (data_length / SECTOR_SIZE + 1) * SECTOR_SIZE;
    }
    let sectors = data_length / SECTOR_SIZE;
    if sector as usize + sectors > SECTOR_COUNT {
        panic!("Multi sector read beyond end of track");
    }

    let completion_code: u8;
    let asynch = request == RBDIN_ASYNC || request == WRTBIN_ASYNC;
    if volume != 0 {
        completion_code = 0xc2 // Disk is not ready
    } else if request == RBDIN_SYNC || request == RBDIN_ASYNC {
        for i in 0..sectors {
            read_disk(machine, data_address, sector + i as u8, track);
            data_address = data_address.wrapping_add(SECTOR_SIZE as u16);
        }
        completion_code = 0x80; // Normal return
    } else if request == WRTBIN_SYNC || request == WRTBIN_ASYNC {
        completion_code = 0xc3; // Disk is write protected
    } else {
        completion_code = 0xc1; // Invalid operation request
    }

    machine.poke(iy+10, completion_code);

    if !asynch {
        0
    } else if completion_code == 0x80 {
        completion_return_address
    } else {
        error_return_address
    }
}

fn interrupt(cpu: &mut Cpu, machine: &mut PdsMachine, dest: u16) {
    let pc = cpu.registers().pc();
    let sp = cpu.registers().get16(Reg16::SP);
    machine.poke(sp-2, pc as u8);
    machine.poke(sp-1, (pc >> 8) as u8);
    cpu.registers().set16(Reg16::SP, sp-2);
    cpu.registers().set_pc(dest);
}

fn main() {
    // Parse arguments
    let matches = App::new(WELCOME)
        .arg(Arg::with_name("cpu_trace")
            .short("c")
            .long("cpu-trace")
            .help("Traces CPU instructions execution"))
        .arg(Arg::with_name("io_trace")
            .short("i")
            .long("io-trace")
            .help("Traces ports IN and OUT"))
        .arg(Arg::with_name("floppy_trace")
            .short("f")
            .long("floppy-trace")
            .help("Traces disk access"))
        .get_matches();

    let trace_cpu = matches.is_present("cpu_trace");
    let trace_io = matches.is_present("io_trace");
    let trace_floppy = matches.is_present("floppy_trace");

    // Init device
    let mut machine = PdsMachine::new(trace_io);
    let mut cpu = Cpu::new_z80();
    cpu.set_trace(trace_cpu);

    // Start the cpu
    println!("{}", WELCOME);

    let done = false;
    let mut async_address: u16 = 0;
    let mut async_count = 0;

    while !done {
        let pc = cpu.registers().pc();
        //cpu.set_trace(trace_cpu && (pc < 0x757 || pc > 0x75b));

        if async_count > 0 {
            async_count -= 1;
            if async_count == 0 {
                interrupt(&mut cpu, &mut machine, async_address);
            }
        }

        if pc == 0x0780 {
            // FLOPPY REQUEST
            if async_count > 0 {
                panic!("Floopy request with a pending async request");
            }

            let iy = cpu.registers().get16(Reg16::IY);
            async_address = rom_floopy(&mut machine, iy, trace_floppy);
            if async_address != 0 {
                async_count = 10000;
            }
            cpu.registers().set_pc(0x0797); // Jump to the RET
        }

        cpu.execute_instruction(&mut machine);

        if cpu.is_halted() {
            println!("HALT instruction that will never be interrupted");
            break;
        }
    }
}

