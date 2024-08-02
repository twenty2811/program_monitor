use program_monitor::ProgramMonitor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("USAGE: {} program_to_monitor <arguments...>", args[0]);
        std::process::exit(1);
    }

    let program_to_monitor = args[1].clone();
    let program_args = args[2..].to_vec();

    let monitor = ProgramMonitor::new(program_to_monitor, program_args);

    if let Err(err) = monitor.monitor_program() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
