use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

const PATH: &str = "hello.txt";

fn main() {
    let file = File::open(PATH);

    let f = file.unwrap_or_else(|y| match y.kind() {
        ErrorKind::NotFound => match File::create(PATH) {
            Ok(z) => z,
            _ => panic!("Couldn't create file"),
        },
        _ => panic!("ERRORR"),
    });
}

fn read_username() -> Result<String, io::Error> {
    let mut file = match File::open(PATH) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_optimized() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(PATH)?.read_to_string(&mut username)?;
    Ok(username)
}

// actually we could have just do this:
fn read_username_even_more_optimized() -> Result<String, io::Error> {
    fs::read_to_string(PATH)
}

fn first_char_of_the_second_line(text: &str) -> Option<char> {
    text.lines().nth(1)?.chars().nth(0)
}
