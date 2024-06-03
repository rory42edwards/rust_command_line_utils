use std::error::Error;
use std::fs::{self, ReadDir};
use std::io::ErrorKind;
use std::process;

pub struct LsConfig {
    pub dir_path: String, // could use an Option<String> here but chose not to as we'll have a
                          // default directory path anyway
}

impl LsConfig {
    pub fn build(args: &[String]) -> Result<LsConfig, &'static str> {
        
        // get compulsory arguments
        let mut dir_path = "./".to_string();
        if args.len() > 1 {
            dir_path = args[1].clone();
        }

        Ok(LsConfig { dir_path })
    } 
}

pub fn run(config: LsConfig) -> Result<(), Box<dyn Error>> {
    let paths_result = fs::read_dir(config.dir_path.clone());

    let paths = match paths_result {
        Ok(read_dir) => read_dir,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("Location not found: {}", config.dir_path);
                process::exit(1);
            },
            other_error => {
                panic!("Send help! {:?}", other_error);
            }
        },
    };
    
    let file_list = make_file_list(paths);

    for file in file_list {
        println!("{file}")
    }

    Ok(())
}

pub fn make_file_list(paths: ReadDir) -> Vec<String>{
    let mut results = Vec::new();
    //let paths = fs::read_dir(config.dir_path);
    for path in paths {
        results.push(path.unwrap().path().display().to_string())
    }
    results.sort(); // ensure consistent order
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_list_dir() {
        let paths = fs::read_dir("/home/rory/Projects/rust/command_line_utils/testing").unwrap();
        let result = make_file_list(paths);

        let mut expected = vec!["/home/rory/Projects/rust/command_line_utils/testing/judo.txt".to_string(), "/home/rory/Projects/rust/command_line_utils/testing/poem.txt".to_string()];
        expected.sort();
        assert_eq!(expected,result);
    }
}
