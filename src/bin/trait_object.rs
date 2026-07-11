use std::any::Any;

trait Waifu {
    fn say_smth(&self);
    fn as_any(&self) -> &dyn Any;
}

struct Reze;
impl Reze {
    fn last_words(&self) {
        println!(
            "Reze: Denji... I wonder why I didn't kill you the first time we met.\nYou know, to be honest... I've never had the chance to go to school either."
        )
    }
}
impl Waifu for Reze {
    fn say_smth(&self) {
        println!("Reze: Hello, Denji!")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Makima;
impl Makima {
    fn control(&self) {
        println!("Makima: From now on, you’re in my care... I expect 'yes' or 'woof' for answers")
    }
}
impl Waifu for Makima {
    fn say_smth(&self) {
        println!("Makima: You seem lonely Denji..")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let mut v = Vec::<Box<dyn Waifu>>::new();
    v.push(Box::new(Reze {}));
    v.push(Box::new(Makima {}));

    v.iter().for_each(|w| {
        w.say_smth();
        if let Some(wany) = w.as_any().downcast_ref::<Reze>() {
            wany.last_words();
        }
        if let Some(wany) = w.as_any().downcast_ref::<Makima>() {
            wany.control();
        }
    });
}
