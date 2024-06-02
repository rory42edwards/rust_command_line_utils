use std::env;
use std::process;
use command_line_utils::rs_ls::LsConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = LsConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = command_line_utils::run_ls(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
