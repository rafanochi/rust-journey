use crate::{Meta, THUMB_CACHE, THUMB_CACHE_FALLBACK, THUMB_IMAGE_FORMAT, ThumSize};
use anyhow::Result;
use magick_rust::{MagickWand, magick_wand_genesis};
use mime::Mime;
use std::{fs, path::Path, sync::Once, time::SystemTime};

pub fn thumbnail(start: &Once, filepath: &str, size: ThumSize) -> Result<Vec<u8>> {
    start.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();

    // reading image
    wand.read_image(filepath)?;
    wand.fit(size.into(), size.into());

    let meta = Meta::fetch_meta(filepath, wand)?;
    for (k, v) in meta.to_hashmap() {
        wand.set_image_property(k, &v);
    }
    let bytes = wand.write_image_blob(THUMB_IMAGE_FORMAT.into())?;
    Ok(bytes)
}

pub fn get_cache_path() -> Result<String> {
    if fs::exists(THUMB_CACHE).is_ok() {
        Ok(THUMB_CACHE.into())
    } else {
        fs::create_dir_all(THUMB_CACHE_FALLBACK)?;
        Ok(THUMB_CACHE_FALLBACK.into())
    }
}

pub fn get_cache_fail_path() -> Result<String> {
    let cache = get_cache_path()?;
    Ok(format!("{cache}/fail"))
}

pub fn find_mimetype(filepath: &Path) -> Option<Mime> {
    filepath
        .file_name()
        .and_then(|name| name.to_string_lossy().to_string().parse::<Mime>().ok())
}
