use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim())
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "stamped";
        // let file_path = "Ozymandias.txt";
        let contents = "\
        Tell that its sculptor well those passions read
        Which yet survive, stamped on these lifeless things,
        The hand that mocked them and the heart that fed.";

        assert_eq!(
            vec!["Which yet survive, stamped on these lifeless things,"],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "sUrviVe";
        let contents = "\
        Tell that its sculptor well those passions read
        Which yet survive, stamped on these lifeless things,
        The hand that mocked them and the heart that fed.";

        assert_eq!(
            vec!["Which yet survive, stamped on these lifeless things,"],
            search_case_insensitive(query, contents)
        );
    }
}
