use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Album {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
}
