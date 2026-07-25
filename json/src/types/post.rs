use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}
