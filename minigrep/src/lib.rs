use std::fs;
use std::error::Error;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(cfg.filename)?;
    for line in search(&cfg.query, &contents) {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }

    }
    results
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_with_valid_args() {
        let args = vec![String::from("minigrep"), String::from("derp"), String::from("poem.txt")];

        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, args[1]);
        assert_eq!(config.filename, args[2]);
    }

    #[test]
    fn config_new_with_too_few_args() {
        let args = vec![String::from("derp"), String::from("poem.txt")];

        let config = Config::new(&args);

        assert!(config.is_err(), "Should fail if too few args are passed in");
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
