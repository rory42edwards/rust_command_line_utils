use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];
    //let file_path = &args[2];

    println!("{input}");
    //println!("In file {}", file_path);
}
