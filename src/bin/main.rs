use minigrep::run;
use minigrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get the filename from the command line arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    // println!("Searching for {}", config.query);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}
