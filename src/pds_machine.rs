use iz80::Machine;

/* Memory map:

    0x0000-0x0cff: 3Kb of ROM
    Rest: RAM for now

*/


const IO_PORT_NAMES: [&str; 32] = [
    /* 0x00 */"Baud rate A, serial",
    /* 0x01 */"-",
    /* 0x02 */"-",
    /* 0x03 */"-",
    /* 0x04 */"SIO A data register.",
    /* 0x05 */"SIO B data register, keyboard.",
    /* 0x06 */"SIO A control register.",
    /* 0x07 */"SIO B control register, keyboard.",
    /* 0x08 */"PIO 1 channel A data register.",
    /* 0x09 */"PIO 1 channel A control register.",
    /* 0x0a */"PIO 1 channel B data register.",
    /* 0x0b */"PIO 1 channel B control register.",
    /* 0x0c */"Baud rate B, keyboard.",
    /* 0x0d */"-",
    /* 0x0e */"-",
    /* 0x0f */"-",
    /* 0x10 */"Floppy controller, Command/status register.",
    /* 0x11 */"Floppy controller, Track register.",
    /* 0x12 */"Floppy controller, Sector register.",
    /* 0x13 */"Floppy controller, Data register.",
    /* 0x14 */"-",
    /* 0x15 */"-",
    /* 0x16 */"-",
    /* 0x17 */"-",
    /* 0x18 */"-",
    /* 0x19 */"-",
    /* 0x1a */"-",
    /* 0x1b */"-",
    /* 0x1c */"PIO 2 channel A data register: ",
    /* 0x1d */"PIO 2 channel A control register.",
    /* 0x1e */"PIO 2 channel B data register.",
    /* 0x1f */"PIO 2 channel B control register.",
    ];


static ROM: &[u8] = include_bytes!("../rom/MCZ.PROM.78089.BIN");

pub struct PdsMachine {
    ram: [u8; 65536],
    trace_io: bool,
}

impl PdsMachine {
    pub fn new(trace_io: bool) -> PdsMachine {
        PdsMachine {
            ram: [0; 65536],
            trace_io,
        }
    }
}

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
            println!("OUT(0x{:02x} '{}', 0x{:02x}): ", port, "todo", value) //IO_PORT_NAMES[port as usize], value);
        }

        match port {
            //0x1c => self.update_system_bits(value),
            _ => {}
        } 
    }

    fn port_in(&mut self, address: u16) -> u8 {
        let port = address as u8 & 0b_1111_1111; // Pins used

        let value = match port {
            //0x05 => self.keyboard.get_key(),
            _ => 0xca,
        }; 

        if self.trace_io {
            println!("IN(0x{:02x} '{}') = 0x{:02x}", port, "todo", value) //IO_PORT_NAMES[port as usize], value);
        }
        value
    }
}
