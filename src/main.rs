use std::env; // Import std::env to read Environment variables

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args {:?}", args);
}
