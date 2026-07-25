use std::{fmt::Error, fs};

use json::User;

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("assets/users.json")?;
    let users: Vec<User> = serde_json::from_str(&content)?;
    users.into_iter().for_each(|x| println!("{x:?}"));
    Ok(())
}
