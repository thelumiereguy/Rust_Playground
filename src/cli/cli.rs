use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[2].clone();
    println!("Args are {:?}", command);
}
