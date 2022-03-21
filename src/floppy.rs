use iz80::Machine;

use super::mcz_machine::*;
use super::media::*;

const RBDIN_SYNC: u8 = 0x0a;
const RBDIN_ASYNC: u8 = 0x0b;
const WRTBIN_SYNC: u8 = 0x0e;
const WRTBIN_ASYNC: u8 = 0x0f;

fn read_disk_sector(machine: &mut MczMachine, media: &Media, address: u16, sector: u8, track: u8) {
    let data = media.read_sector(track as usize, sector as usize);
    for i in 0..SECTOR_SIZE {
        machine.poke(address+i as u16, data[2+i]);
    }

    // Store the pointers and CRC on the PROM working memory, RIO OS reads those bytes.
    for i in 0..6 {
        machine.poke(FLOPPY_POINTERS+i as u16, data[2+SECTOR_SIZE+i]);
    }
}

fn write_disk_sector(machine: &mut MczMachine, media: &mut Media, address: u16, sector: u8, track: u8) {
    let mut data = [0; SECTOR_SIZE_IN_FILE];
    data[0] = sector & 0x80;
    data[1] = track;
    for i in 0..SECTOR_SIZE {
        data[2+i] = machine.peek(address+i as u16);
    }

    // Get the pointers and CRC on the PROM working memory, RIO OS write those bytes.
    for i in 0..6 {
        data[2+SECTOR_SIZE+i] = machine.peek(FLOPPY_POINTERS+i as u16);
    }
    media.write_sector(track as usize, sector as usize, &data).unwrap();
}

pub fn rom_floopy(machine: &mut MczMachine, drives: &mut[Media], iy: u16, floppy_trace: bool) -> u16 {
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
    if volume >= drives.len() as u8 {
        completion_code = 0xc2 // Disk is not ready
    } else if request == RBDIN_SYNC || request == RBDIN_ASYNC {
        let media = &drives[volume as usize];
        for i in 0..sectors {
            read_disk_sector(machine, media, data_address, sector + i as u8, track);
            data_address = data_address.wrapping_add(SECTOR_SIZE as u16);
        }
        completion_code = 0x80; // Normal return
    } else if request == WRTBIN_SYNC || request == WRTBIN_ASYNC {
        let media = &mut drives[volume as usize];
        for i in 0..sectors {
            write_disk_sector(machine, media, data_address, sector + i as u8, track);
            data_address = data_address.wrapping_add(SECTOR_SIZE as u16);
        }
        completion_code = 0x80; // Normal return
        // Todo, support WP: completion_code = 0xc3; // Disk is write protected
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