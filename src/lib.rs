use config::Config;
use std::error::Error;
use std::fs;

pub mod config;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(&config.file_path).expect("Something went wrong reading the file");

    if config.case_insensitive {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

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
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_result() {
        let query = "james";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let empty_vec: Vec<&str> = Vec::new();

        assert_eq!(empty_vec, search(query, contents));
    }
}
