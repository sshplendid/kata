use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = search(&config.query, &contents);
    print(lines);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        // do something here
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn print(lines: Vec<&str>) -> () {
    for line in lines.iter() {
        println!("{}", line);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn when_basic_test_is_given_perg_return_right_answer() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn when_another_test_is_given_perg_return_right_answer() {
        let query = "e";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three.",],
            search(query, contents)
        );
    }
}
