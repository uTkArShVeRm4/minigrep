use core::str;
use regex::Regex;
use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else if config.re {
        search_regex(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search_regex<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let regex = Regex::new(query).unwrap();
    contents.lines().filter(|x| regex.is_match(x)).collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(&query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub re: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get arguments"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let re = env::var("REGEX").is_ok();
        return Ok(Config {
            query,
            file_path,
            ignore_case,
            re,
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
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

    #[test]
    fn regex_search() {
        let query = r"\b\d{3}\b";
        let contents = "\
1234
567
89
0";
        assert_eq!(vec!["567"], search_regex(&query, &contents));
    }
}
