use std::env;
use std::process;

use command_line_utils::Config; // bring the Config type from library crate into binary crate's scope

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    if let Err(e) = command_line_utils::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
