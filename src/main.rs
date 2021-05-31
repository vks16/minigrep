use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = match Config::new(&args) {
    //     Ok(result) => result,
    //     Err(str) => {
    //         println!("Error: {}", str);
    //         process::exit(1);
    //     }
    // };

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Error, {}", err);
        process::exit(1);
    }
}
