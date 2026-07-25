use std::{fmt::Error, fs};

use json::types::{post::Post, user::User};

fn main() -> Result<(), std::io::Error> {
    let user_file = fs::read_to_string("assets/users.json")?;
    let users: Vec<User> = serde_json::from_str(&user_file)?;

    let user_file = fs::read_to_string("assets/posts.json")?;
    let posts: Vec<Post> = serde_json::from_str(&user_file)?;

    users.into_iter().for_each(|x| println!("{x:?}"));
    posts.into_iter().for_each(|x| println!("{x:?}"));
    Ok(())
}
