use std::env;
use std::process;

use minigrep;

fn main() {
    if let Err(e) = minigrep::run(&env::args().collect()) {
        println!("Fatal error: {}", e);
        process::exit(1);
    }
}
