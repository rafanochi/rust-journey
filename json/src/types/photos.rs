use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Photo {
    #[serde(rename = "albumId")]
    album_id: u32,
    id: u32,
    title: String,
    url: String,
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String,
}
