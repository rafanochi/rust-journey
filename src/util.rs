use crate::{Meta, THUMB_CACHE, THUMB_CACHE_FALLBACK, THUMB_IMAGE_FORMAT, ThumSize};
use magick_rust::{MagickError, MagickWand, magick_wand_genesis};
use mime::{Mime, TEXT_PLAIN};
use std::{
    fs::{self, Metadata},
    sync::Once,
    time::SystemTime,
};

pub fn thumbnail(start: &Once, filepath: &str, size: ThumSize) -> Result<Vec<u8>, MagickError> {
    start.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();

    let meta = fs::metadata(filepath).map_err(|e| MagickError(e.to_string()))?;
    dbg!(&meta.file_type());
    println!("{:#?}", meta.file_type());

    let mtime = meta.modified().map_err(|e| MagickError(e.to_string()))?;
    let seconds_since_mtime = mtime
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|msg| MagickError(msg.to_string()))?
        .as_secs();

    // reading image
    wand.read_image(filepath)?;
    wand.fit(size.into(), size.into());
    wand.set_image_property("Thumb::URI", filepath)?;
    wand.set_image_property("Thumb::MTime", &seconds_since_mtime.to_string())?;
    wand.get_image_properties("date")
        .iter()
        .for_each(|x| println!("{x:?}"));
    wand.set_image_property("Description", "alksjf;laksjdf;alksjdf;laksjdf;alkjsef")?;

    wand.write_image_blob(THUMB_IMAGE_FORMAT.into())
}

// pub fn get_meta(filepath: &str) -> Meta {
//     let mut meta
// }

pub fn get_cache_path() -> anyhow::Result<String> {
    if fs::exists(THUMB_CACHE).is_ok() {
        Ok(THUMB_CACHE.into())
    } else {
        fs::create_dir_all(THUMB_CACHE_FALLBACK)?;
        Ok(THUMB_CACHE_FALLBACK.into())
    }
}

pub fn get_cache_fail_path() -> anyhow::Result<String> {
    let cache = get_cache_path()?;
    Ok(format!("{cache}/fail"))
}

pub fn find_mimetype(filename: &str) -> Option<Mime> {
    let parts: Vec<&str> = filename.split('.').collect();

    parts.last().and_then(|last| last.parse::<Mime>().ok())
}
