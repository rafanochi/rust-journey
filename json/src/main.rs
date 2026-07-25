use std::{fmt::Error, fs};

use json::types::{album::Album, comment::Comment, photos::Photo, post::Post, user::User};

fn main() -> Result<(), std::io::Error> {
    let user_file = fs::read_to_string("assets/users.json")?;
    let users: Vec<User> = serde_json::from_str(&user_file)?;

    let post_file = fs::read_to_string("assets/posts.json")?;
    let posts: Vec<Post> = serde_json::from_str(&post_file)?;

    let comment_file = fs::read_to_string("assets/comments.json")?;
    let comments: Vec<Comment> = serde_json::from_str(&comment_file)?;

    let album_file = fs::read_to_string("assets/albums.json")?;
    let albums: Vec<Album> = serde_json::from_str(&album_file)?;

    let photo_file = fs::read_to_string("assets/photos.json")?;
    let photos: Vec<Photo> = serde_json::from_str(&photo_file)?;

    users.into_iter().for_each(|x| println!("{x:?}"));
    posts.into_iter().for_each(|x| println!("{x:?}"));
    comments.into_iter().for_each(|x| println!("{x:?}"));
    albums.into_iter().for_each(|x| println!("{x:?}"));
    photos.into_iter().for_each(|x| println!("{x:?}"));
    Ok(())
}
