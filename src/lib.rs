use std::env; // needed for handling environment variables (will change to proper command line
              // flags later

// define run functions for each binary
pub mod rs_grep;
pub mod rs_echo;

pub use rs_grep::run as run_grep;
pub use rs_echo::run as run_echo;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub ignore_case_flag: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // get compulsory arguments
        let query = args[1].clone();
        let file_path = args[2].clone();

        // check for case insensitivity flag
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let mut ignore_case_flag = None;

        if (args.len() > 3 && args[3] == "--ignore-case") || (args.len() > 3 && args[3] == "-i") {
            ignore_case_flag = Some(args[3].clone());
        }

        Ok(Config { query, file_path, ignore_case, ignore_case_flag})
    } 
}
