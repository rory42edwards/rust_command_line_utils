use crate::Config;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input = &config.query;
    println!("{input}");

    Ok(())
}
