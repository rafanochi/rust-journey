use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}
