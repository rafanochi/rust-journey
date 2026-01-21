use std::env;

struct Config<'a> {
    text: &'a str,
    path: &'a str,
}
impl Config<'_> {
    fn build(args: &[String]) -> Result<Config<'_>, impl Eq> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            text: &args[1],
            path: &args[2],
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    // let contents = fs::read_to_string(config.unwrap().path).expect("couln't read the file");

    // println!("{contents}");
}
