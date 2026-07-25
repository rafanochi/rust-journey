use std::{fmt::Error, fs};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Deserialize, Debug)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Deserialize, Debug)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Deserialize, Debug)]
struct Company {
    name: String,
    #[serde(rename = "catchPhrase")]
    catch_phrase: String,
    bs: String,
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("assets/users.json")?;
    let users: Vec<User> = serde_json::from_str(&content)?;
    users.into_iter().for_each(|x| println!("{x:?}"));
    Ok(())
}
