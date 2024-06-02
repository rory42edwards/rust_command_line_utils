use std::env;
use std::process;
use command_line_utils::rs_echo::EchoConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let input = &args[1];
    let config = EchoConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("{input}");
    if let Err(e) = command_line_utils::run_echo(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
