use minigrep::search;
use std::{env, error::Error, fs, process::exit};

struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}
impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("no query"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

fn run(c: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&c.path)?;

    for line in search(&c.query, &contents) {
        println!("{line:?}");
    }

    Ok(())
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {err:?}");
        exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        exit(1);
    };

    // match run(&config){
    //    Ok(result) => println!("{result}"),
    //    Err(e) => {}
    // }
}
