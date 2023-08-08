use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // Get command arguments
    let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
