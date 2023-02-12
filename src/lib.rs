use std::error::Error;
use std::{env, fs};
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    let res = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        case_insensitive_search(&config.query, &contents)
    };
    for line in res {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lines = contents.lines().filter(|line| line.contains(query));
    lines.collect()
}
fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let lines = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query));
    lines.collect()
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let option_case_insensitive = args[3].clone();
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let mut case_sensitive = true;
        if option_case_insensitive == "-i" {
            case_sensitive = false
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod test {
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
}
