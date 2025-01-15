use std::{env, process};

use ch12_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // using if here is more convenient than using unwrap_or_else becuase we only care about Err
    if let Err(e) = ch12_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
