pub struct Waifu {
    pub name: String,
    age: i32,
}

impl Waifu {
    pub fn make_child(child_name: &str) -> Self {
        Waifu {
            name: child_name.to_string(),
            age: 0,
        }
    }
}

pub fn add_to_waitlist() {}
fn seat_at_table() {}

pub mod serving {
    use std::collections::HashMap;

    pub fn cook() {
        let mut map = HashMap::new();
        map.insert("birinchi_ovqat", "kakasha");
    }
    fn bring_meal() {}
    fn clean_table() {}
}

pub fn say_hello() {
    let w = Waifu::make_child;
}
