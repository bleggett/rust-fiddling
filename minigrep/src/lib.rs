use std::fs;
use std::error::Error;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(cfg.filename)?;
    println!("Contents: {}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config { query: args[1].clone(), filename: args[2].clone() })
    }
}
