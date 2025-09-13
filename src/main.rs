use std::{env, fs, process, error::Error};
use simple_grep::{search, search_case_insensitive};
// cargo run -- searchstring example-filename.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
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
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = if args.contains(&"-i".to_string()) { true } else { env::var("IGNORE_CASE").is_ok() };
        Ok(Config { query, file_path, ignore_case })
    }
}
