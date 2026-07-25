use std::fs;

use json::types::{
    album::Album, comment::Comment, photos::Photo, post::Post, todo::Todo, user::User,
};

fn main() -> Result<(), std::io::Error> {
    fs::read_dir("assets")?
        .into_iter()
        // .filter_map(|x| match x {
        //     Ok(y) => fs::read_to_string(y.path()).ok(),
        //     Err(_) => None,
        // })
        .filter_map(|x| x.map_or(None, |y| fs::read_to_string(y.path()).ok())) // rewrite the old filter_map
        .for_each(|path| {
            if let Some(x) = serde_json::from_str::<Vec<User>>(&path).ok() {
                println!("{x:?}")
            }
            if let Some(x) = serde_json::from_str::<Vec<Post>>(&path).ok() {
                println!("{x:?}")
            }
            if let Some(x) = serde_json::from_str::<Vec<Comment>>(&path).ok() {
                println!("{x:?}")
            }
            if let Some(x) = serde_json::from_str::<Vec<Album>>(&path).ok() {
                println!("{x:?}")
            }
            if let Some(x) = serde_json::from_str::<Vec<Photo>>(&path).ok() {
                println!("{x:?}")
            }
            if let Some(x) = serde_json::from_str::<Vec<Todo>>(&path).ok() {
                println!("{x:?}")
            }
        });
    Ok(())
}
