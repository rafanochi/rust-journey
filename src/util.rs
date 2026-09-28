use crate::{Meta, THUMB_CACHE, THUMB_CACHE_FALLBACK, THUMB_IMAGE_FORMAT, ThumSize};
use anyhow::Result;
use magick_rust::{MagickWand, magick_wand_genesis};
use mime::Mime;
use std::{fs, path::Path, sync::Once, time::SystemTime};

pub fn thumbnail(filepath: &str, size: ThumSize) -> Result<String> {
    // initialize MagickWand to create thumbnail
    let start: Once = Once::new();
    start.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();

    // read and resize image
    wand.read_image(filepath)?;
    wand.fit(size.into(), size.into());

    // add metadata to thumbnail
    let path = Path::new(filepath);
    let meta = Meta::fetch_meta(path, &wand)?;
    for (k, v) in meta.to_hashmap() {
        wand.set_image_property(k, &v)?;
    }
    let bytes = wand.write_image_blob(THUMB_IMAGE_FORMAT.into())?;

    let filename = ;
    fs::write(get_cache_path(path.file_name()), bytes).expect("write failed");

    Ok(filepath.into())
}

pub fn get_cache_path(filename: &str) -> Result<String> {
    let path = if fs::exists(THUMB_CACHE).is_ok() {
        THUMB_CACHE
    } else {
        fs::create_dir_all(THUMB_CACHE_FALLBACK)?;
        THUMB_CACHE_FALLBACK
    };

    Ok(format!("{path}/{filename}"))
}

pub fn get_cache_fail_path(filename: &str) -> Result<String> {
    let cache = get_cache_path("")?;
    Ok(format!("{cache}/fail{filename}"))
}

pub fn find_mimetype(filepath: &Path) -> Option<Mime> {
    filepath
        .file_name()
        .and_then(|name| name.to_string_lossy().to_string().parse::<Mime>().ok())
}
