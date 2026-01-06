trait Love {
    fn confess(&self, name: &str);
    fn confess_to(name: &str) -> String {
        format!("I love you {name}")
    }
}

struct Waifu {
    name: String,
    age: u8,
}
impl Love for Waifu {
    fn confess(&self, name: &str) {
        println!("{}: {}", &self.name, &Waifu::confess_to(name))
    }
}

// fn confess<T: Love>(x: &T, name: &str) {
fn confess(x: &impl Love, name: &str) {
    x.confess(name);
}

fn main() {
    let asuna = Waifu {
        name: String::from("Asuna"),
        age: 19,
    };
    confess(&asuna, "Rafa");
}
