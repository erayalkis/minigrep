use std::env; // Import std::env to read Environment variables
use std::fs; // Import std::fs to read and write files
use std::process; // Import process to exit with code

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments: {}", err);
        process::exit(1);
    });

    let content = fs::read_to_string(config.filename).expect("Could not read file.");
    println!("{}", content);
}
