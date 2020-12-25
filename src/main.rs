use std::{env, process};

use hello_rust::minigrep::{Config, run};

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // env::var("").is

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
