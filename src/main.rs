use std::{env, fs, process, error::Error};
use simple_grep::{search, search_case_insensitive};
// cargo run -- searchstring example-filename.txt
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error Parsing arguments {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application Error{err}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in lines {
        println!("{line}");
    }
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip first argument
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get path string")
            
        };
        let ignore_case = match args.next() {
            Some(input) => {
                if input == "-i" {true} else {false}
            },
            None => env::var("IGNORE_CASE").is_ok()
        };
        Ok(Config { query, file_path, ignore_case })
    }
}
