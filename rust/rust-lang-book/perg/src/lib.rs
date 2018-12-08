extern crate colored;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use colored::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let mut case_sensitive = true;
        if args.len() > 3 && args[3].contains("-c") {
            case_sensitive = false;
        }

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
    print_lines(lines, &config.query);

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

fn print_lines(lines: Vec<&str>, query: &str) -> () {
    for line in lines.iter() {
        print(&line, &query);
    }
}

fn print(line: &str, query: &str) -> () {
    let v: Vec<&str> = line.split(&query).collect();

    print!("{}", v[0]);
    for i in (1..v.len()) {
        print!("{}", query.blue().reverse());
        print!("{}", v[i]);
    }
    println!("");
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

    #[test]
    fn colored_test() {
        let green = "it works".green();
        let reverse_blue = "awesome colored!".blue().reverse();

        println!("{} {}", green, reverse_blue);
        //println!("{} {}", green.compute_style(), reverse_blue.compute_style());
    }

    #[test]
    fn split_test() {
        let line = "This apple is awesome!";
        let pattern = "a";

        let v: Vec<&str> = line.split(&pattern).collect();

        print!("{}", v[0]);
        for i in (1..v.len()) {
            print!("{}", pattern.blue().reverse());
            print!("{}", v[i]);
        }
        println!("");

        assert_eq!(v, ["This ", "pple is ", "wesome!"]);
    }
}
