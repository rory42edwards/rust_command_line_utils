use std::env;
use std::error::Error;

pub struct EchoConfig {
    pub input: String,
    pub ignore_case: bool,
    pub ignore_case_flag: Option<String>,
}

impl EchoConfig {
    pub fn build(args: &[String]) -> Result<EchoConfig, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        // get compulsory arguments
        let input = args[1].clone();

        // check for case insensitivity flag
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let mut ignore_case_flag = None;

        if (args.len() > 2 && args[2] == "--ignore-case") || (args.len() > 2 && args[2] == "-i") {
            ignore_case_flag = Some(args[2].clone());
        }

        Ok(EchoConfig { input, ignore_case, ignore_case_flag})
    } 
}

pub fn run(config: EchoConfig) -> Result<(), Box<dyn Error>> {
    let input = &config.input;

    let results = if config.ignore_case_flag.is_some() {
        input.to_lowercase()
    } else {
        input.to_string()
    };
    println!("{results}");

    Ok(())
}
