trait Waifu {
    fn say_smth(&self);
}

struct Reze;
impl Waifu for Reze {
    fn say_smth(&self) {
        println!("Hello, Denji!")
    }
}

struct Makima;
impl Waifu for Makima {
    fn say_smth(&self) {
        print!("You seem lonely Denji..")
    }
}

fn main() {
    let mut v = Vec::<Box<dyn Waifu>>::new();
    v.push(Box::new(Reze {}));
    v.push(Box::new(Makima {}));

    v.iter().for_each(|w| w.say_smth());
}
