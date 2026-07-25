use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Comment {
    #[serde(rename = "postId")]
    post_id: u32,
    id: u32,
    name: String,
    email: String,
    body: String,
}
