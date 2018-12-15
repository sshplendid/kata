extern crate perg;

use perg::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = perg::run(config) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    };
}
