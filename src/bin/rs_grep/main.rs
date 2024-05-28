use std::env;
use std::fs; // needed to handle files

fn main(){
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
