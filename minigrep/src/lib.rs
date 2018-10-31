use std::fs;
use std::env;
use std::error::Error;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(cfg.filename)?;

    let results = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string found")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename string found")
        };

        Ok(Config {
            query,
            filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
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
    fn case_sensitive_one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive_one_result() {
        let query = "RuSt";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
