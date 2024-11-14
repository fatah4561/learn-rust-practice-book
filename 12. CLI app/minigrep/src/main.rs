use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error on processing arguments: {err}");
        process::exit(1)
    });

    // println!("query is {}", config.query);
    // println!("file_path is {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Failed to run the app: {e}");
        process::exit(1); // other than 0 means error
    };

}