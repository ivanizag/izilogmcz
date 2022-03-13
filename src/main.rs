use clap::{Arg, App};
use iz80::*;

mod pds_machine;

use self::pds_machine::PdsMachine;

// Welcome message
const WELCOME: &str =
"Kaypro https://github.com/ivanizag/izkaypro
Emulation of the Zilog MCZ-1 computer";


fn main() {
    // Parse arguments
    let matches = App::new(WELCOME)
        .arg(Arg::with_name("cpu_trace")
            .short("c")
            .long("cpu-trace")
            .help("Traces CPU instructions execuions"))
        .arg(Arg::with_name("io_trace")
            .short("i")
            .long("io-trace")
            .help("Traces ports IN and OUT"))
        .get_matches();

    let trace_cpu = matches.is_present("cpu_trace");
    let trace_io = matches.is_present("io_trace");

    // Init device
    let mut machine = PdsMachine::new(trace_io);
    let mut cpu = Cpu::new_z80();
    cpu.set_trace(trace_cpu);

    // Start the cpu
    println!("{}", WELCOME);

    let done = false;
    while !done {
        cpu.execute_instruction(&mut machine);

        if cpu.is_halted() {
            println!("HALT instruction that will never be interrupted");
            break;
        }

    }
}

