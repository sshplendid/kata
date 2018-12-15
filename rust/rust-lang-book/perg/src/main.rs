extern crate perg;

use std::env;
use std::process;
use perg::Config;

fn main() {
    /*
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    */
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = perg::run(config) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    };
}
