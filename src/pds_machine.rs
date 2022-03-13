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

pub struct PdsMachine {
    ram: [u8; 65536],
    trace_io: bool,
    console: Console,

    clk1: u8, // TTY timer
    //i_command: usize
}

impl PdsMachine {
    pub fn new(trace_io: bool) -> PdsMachine {
        PdsMachine {
            ram: [0; 65536],
            trace_io,
            console: Console::new(),

            clk1: 0,
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

        if self.trace_io {
            println!("OUT(0x{:02x} '{}', 0x{:02x}): ", port, port_name(port), value)
        }

        match port {
            0xd5 /*CLK1  */ => self.clk1 = value,
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
        0xCF => "DSKDAT",
        0xD0 => "DSKCOM0",
        0xD1 => "DSKSEL0",
        0xD2 => "DSKCOM1",
        0xD3 => "DSKSEL1",
        //0xD0 => "DSSTAT",

        0xD4 => "CLK0  ",
        0xD5 => "CLK1  ",
        0xD7 => "BRKPRT",

        0xDD => "SWITCH", // Configuration jumpers, the 4 LSB are the tty speed
        0xDE => "SERDAT",
        0xDF => "SERCON", // Serial port control
                          // Bit0: transfer ready. Always true.
                          // Bit1: receieve ready
        _ => "unknown"
    }
}