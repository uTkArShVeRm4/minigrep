use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|_err| {
        println!("Problem parsing the arguments");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}
