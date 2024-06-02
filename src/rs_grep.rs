use std::env; // needed for handling environment variables
use std::error::Error; // needed for error handling (surprise surprise)
use std::fs; // needed to handle files
             
pub struct GrepConfig {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub ignore_case_flag: Option<String>,
}

impl GrepConfig {
    pub fn build(args: &[String]) -> Result<GrepConfig, &'static str> {
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

        Ok(GrepConfig { query, file_path, ignore_case, ignore_case_flag})
    } 
}

pub fn run(config: GrepConfig) -> Result<(), Box<dyn Error>> {
    //let default_path = "/home/rory".to_string();
    //let file_path = config.file_path.clone();
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case_flag.is_some() {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
