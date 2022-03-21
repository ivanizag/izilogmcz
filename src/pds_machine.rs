//use std::io::stdout;
//use std::io::Write;

use iz80::Machine;

#[cfg(unix)]
use super::console_unix::Console;


/* Memory map:

    0x0000-0x0cff: 3Kb of ROM
    Rest: RAM for now

*/

static ROM: &[u8] = include_bytes!("../rom/MCZ.PROM.78089.BIN");
pub const FLOPPY_HANDLER: u16 = 0x0780;
pub const FLOPPY_POINTERS: u16 = 0x12b4;

pub struct PdsMachine {
    ram: [u8; 65536],
    trace_io: bool,
    console: Console,

    //i_command: usize
}

impl PdsMachine {
    pub fn new(trace_io: bool) -> PdsMachine {
        PdsMachine {
            ram: [0; 65536],
            trace_io,
            console: Console::new(),

            //i_command: 0
        }
    }

    fn is_key_ready(&mut self) -> bool {
        //self.i_command < COMMAND.len()
        self.console.status()
    }

    fn get_key(&mut self) -> u8 {
        self.console.read()
        /*
        if self.is_key_ready() {
            self.i_command += 1;
        }
        COMMAND[self.i_command - 1]
        */
    }

    fn put_char(&mut self, ch: u8) {
        self.console.put(ch);
        /*
        print!("{}", ch as char);
        stdout().flush().unwrap();
        */
    }
}

//const COMMAND: &[u8] = " HELP\r".as_bytes();

impl Machine for PdsMachine {
    fn peek(&self, address: u16) -> u8 {
        //if address >= 0x1100 && address <= 0x1300 {
        //    print!("Access to {:04x}h\n", address);
        //}

        if address < ROM.len() as u16 {
            ROM[address as usize]
        } else {
            self.ram[address as usize]
        }
    }

    fn poke(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
        // Note: write to ROM area won't be peekable
    }

    fn port_out(&mut self, address: u16, value: u8) {
        let port = address as u8 & 0b_1111_1111; // Pins used

        if self.trace_io && port != 0xde {
            println!("OUT(0x{:02x} '{}', 0x{:02x})", port, port_name(port), value)
        }

        match port {
//            0xd5 /*CLK1  */ => self.clk1 = value,
            0xde /*SERDAT*/ => self.put_char(value),
            _ => {}
        } 
    }

    fn port_in(&mut self, address: u16) -> u8 {
        let port = address as u8 & 0b_1111_1111; // Pins used

        let value = match port {
            0xdd /*SWITCH*/ => 10, // Baud rate jumpers to 4800 baud
            0xde /*SERDAT*/ => self.get_key(),
            0xdf /*SERCON*/ =>
                1 /* TXREADY */
                | if self.is_key_ready() {2} else {0} /* RXREADY */,
            //0x05 => self.keyboard.get_key(),

            0xd7 => 0xff, // CLK3, to pass hardware check

            _ => 0xbb,
        }; 

        if self.trace_io && port != 0xdf {
            println!("IN(0x{:02x} '{}') = 0x{:02x}", port, port_name(port), value)
        }
        value
    }
}

fn port_name(port: u8) -> &'static str {
    match port {
        // Disk controller ports
        0xCF => "DSKDAT",
        0xD0 => "DSKCOM/DSSTAT",
            // bit 0 (OUT): DIRECT (increase track?)
            // bit 1 (OUT): HDSTEP (decrease track?)
            // bit 5 (IN): READY, disk ready
            // bit 6 (IN): TO, track 0
            // bit 7 (IN): CRC, crc error
        0xD1 => "DSKSEL",
            // bits 2-1-0: disk number from 0 to 7
            // bit 3: any disk selected (or seledt/deselect on OUT)
            // bit 6: is disk attached
            // bit 7: WRTPTC, is disk write protected
        0xD2 => "DSKCOM1",
        0xD3 => "DSKSEL1",

        // Z80-CTC ports 0xD4 to 0xD7
        0xD4 => "CLK0", // Channel 0: Used for floppy timing
        0xD5 => "CLK1", // Channel 1: USART clock.
        0xD6 => "CLK2", // Channel 2: User defined
        0xD7 => "CLK3", // Channel 3: User defined BRKPORT

        // Z80-PIO ports 0xD8 to 0xDB

        // Other
        0xDD => "SWITCH", // Configuration jumpers, the 4 LSB are the tty speed

        // USART 8251 ports 0xDE to 0xDF
        0xDE => "SERDAT",
        0xDF => "SERCON", // Serial port control
                          // bit 0 (IN): transfer ready. Always true.
                          // bit 1 (IN): receieve ready
        _ => "unknown"
    }
}