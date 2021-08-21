use std::fs;
use std::error::Error;
use std::env;

pub struct case_opt {
    pub env: bool,
    pub arg: bool
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: case_opt
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut opt: usize = 0;
        for i in args {
            if i.contains("-r") {
                opt = opt + 1;
            }
        }
        if args.len() - opt < 3 {
            return Err("Usage: minigrep <query> <filename>");
        }
        let query = args[1 + opt].clone();
        let filename = args[2 + opt].clone();

        let env = env::var("CASE_INSENSITIVE").is_err();
        let arg = if opt == 1 {
            true
        } else {
            false
        };
        let case_sensitive = case_opt {env,arg};
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive.env == false && config.case_sensitive.arg == false {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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

}