use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

//const MAX_ARGS:u32 = 4;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments!");
        }

        let case_sensitive = args[3].contains("-c");
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = search_with_case_insensitive(&config.query, &contents, config.case_sensitive);
    print(lines);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search_with_case_insensitive(&query, &contents, true)
}
pub fn search_with_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
    case_sensitive: bool,
) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        // do something here
        if is_fit(query, line, case_sensitive) {
            results.push(line);
        }
    }

    results
}

fn is_fit(query: &str, line: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        if line.contains(query) {
            return true;
        }
    } else {
        if convert_case(line).contains(convert_case(query).as_str()) {
            return true;
        }
    }
    false
}

fn convert_case(str_to_convert: &str) -> String {
    let result = str_to_convert.to_lowercase();

    result
}

fn print(lines: Vec<&str>) -> () {
    for line in lines.iter() {
        println!("{}", line);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_fixture<'a>() -> &'a str {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";

        contents
    }
    #[test]
    fn case_default() {
        let query = "duct";
        let contents = get_fixture();

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_another() {
        let query = "ust";
        let contents = get_fixture();

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents));
    }

    #[test]
    fn case_capital_sensitive() {
        let query = "Rust";
        let contents = get_fixture();
        let case_sensitive = true;

        assert_eq!(
            vec!["Rust:"],
            search_with_case_insensitive(query, contents, case_sensitive)
        );
    }

    #[test]
    fn case_capital_insensitive() {
        let query = "Rust";
        let contents = get_fixture();
        let case_sensitive = false;

        assert_eq!(
            vec!["Rust:", "Trust me.",],
            search_with_case_insensitive(query, contents, case_sensitive)
        );
    }
}
