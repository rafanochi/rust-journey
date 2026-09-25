use std::fs;

use anyhow::Ok;

static THUMB_CACHE: &str = "$XDG_CACHE_HOME/thumbnails";
static THUMB_CACHE_FALLBACK: &str = "$HOME/.cache/thumbnails";
static THUMB_IMAGE_FORMAT: &str = "png";

pub enum ThumSize {
    Normal,
    Large,
    XLarge,
    XXLarge,
}
impl ThumSize {
    fn path(&self) -> &str {
        use ThumSize::*;
        match self {
            Normal => "normal",
            Large => "large",
            XLarge => "x-large",
            XXLarge => "xx-large",
        }
    }
}
impl From<ThumSize> for u16 {
    fn from(value: ThumSize) -> Self {
        use ThumSize::*;
        match value {
            Normal => 128,
            Large => 256,
            XLarge => 512,
            XXLarge => 1024,
        }
    }
}

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
