use minigrep::search;
use std::{env, error::Error, fs, process::exit};

struct Config<'a> {
    query: &'a str,
    path: &'a str,
    ignore_case: bool,
}
impl Config<'_> {
    fn build(args: &[String]) -> Result<Config<'_>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: &args[1],
            path: &args[2],
            ignore_case: ignore_case,
        })
    }
}

fn run(c: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(c.path)?;

    for line in search(c.query, &contents) {
        println!("{line:?}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
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
