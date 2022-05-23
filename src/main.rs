use std::env; // Import std::env to read Environment variables
use std::fs; // Import std::fs to read and write files

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let content = fs::read_to_string(config.filename).expect("Could not read file.");
    println!("{}", content);
}
