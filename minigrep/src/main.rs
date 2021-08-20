use std::env;
use std::process;

use minigrep;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|e| {
        println!("Args err: {}", e);
        utils::print_usage();
        process::exit(1);
    });

    if let Err(e) = minigrep::minigrep(config) {
        println!("Fatal error: {}", e);
        process::exit(1);
    }
}
