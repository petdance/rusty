

use std::process;
use std::env;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
