use rand::Rng;

const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic",
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

pub fn random_icon() -> &'static str {
    ICON_LIST
        .iter()
        .nth(rand::thread_rng().gen_range(1..ICON_LIST.len()))
        .expect("ERRORR, item not found")
}

pub fn gen_unique_icon(exclude: &'static str) -> &'static str {
    let mut rnd = random_icon();
    while rnd == exclude {
        rnd = random_icon();
    }
    rnd
}
