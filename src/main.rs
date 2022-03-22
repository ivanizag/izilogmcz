use clap::{Arg, App};
use iz80::*;

mod floppy;
mod media;
mod mcz_machine;

#[cfg(windows)]
mod console_windows;
#[cfg(unix)]
mod console_unix;

use self::floppy::rom_floopy;
use self::media::Media;
use self::mcz_machine::*;

// Welcome message
const WELCOME: &str =
"Emulation of the Zilog MCZ-1 computer
https://github.com/ivanizag/izilogmcz\n";

static DISK_2_2: &[u8] = include_bytes!("../disks/13-3001-03_MCZ-PDS_RIO_2-2.MCZ");
static DISK_2_06: &[u8] = include_bytes!("../disks/13-3001-01_MCZ1-20_RIO_206.MCZ");
static DISK_2_2_SYSTEM: &[u8] = include_bytes!("../disks/13-1000-01-UNABRIDGED_SYSTEM_DISK.MCZ");
static DISK_EMPTY: &[u8] = include_bytes!("../disks/EMPTY.MCZ");

fn interrupt(cpu: &mut Cpu, machine: &mut MczMachine, dest: u16) {
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
        .arg(Arg::with_name("DISK")
            .help("Image file")
            .required(false)
            .multiple(true))
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
    let disks = matches.values_of("DISK");

    // Init device
    let mut machine = MczMachine::new(trace_io);
    let mut cpu = Cpu::new_z80();
    cpu.set_trace(trace_cpu);

    // Load disks
    let mut drives: Vec<Media> = vec![];
    match disks {
        Some(disks) => {
            for disk in disks {
                drives.push(Media::new_from_file(disk).unwrap());
            }
        },
        None => {
            // Load default disks if none specified
            drives.push(Media::new_from_bytes(DISK_2_2));
            drives.push(Media::new_from_bytes(DISK_EMPTY));
            drives.push(Media::new_from_bytes(DISK_2_06));
            drives.push(Media::new_from_bytes(DISK_2_2_SYSTEM));
        }

    }

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

        if pc == FLOPPY_HANDLER {
            // FLOPPY REQUEST
            if async_count > 0 {
                panic!("Floopy request with a pending async request");
            }

            let iy = cpu.registers().get16(Reg16::IY);
            async_address = rom_floopy(&mut machine, &mut drives, iy, trace_floppy);
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

