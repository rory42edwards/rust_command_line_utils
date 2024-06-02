use std::error::Error;
use std::fs;

pub struct LsConfig {
    pub dir_path: String,
}

impl LsConfig {
    pub fn build(args: &[String]) -> Result<LsConfig, &'static str> {
        
        // get compulsory arguments
        let dir_path = args[1].clone();

        Ok(LsConfig { dir_path })
    } 
}

pub fn run(config: LsConfig) -> Result<(), Box<dyn Error>> {
    //let dir_path = config.dir_path;
    let paths = fs::read_dir(config.dir_path).unwrap();
    
    let file_list = make_file_list(paths);

    for file in file_list {
        println!("{file}")
    }

    Ok(())
}

pub fn make_file_list(paths: fs::ReadDir) -> Vec<String>{
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
