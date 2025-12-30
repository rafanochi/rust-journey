#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Person {
    surname: String,
    name: String,
    age: u16, // some waifus are even 500+ years old
}
impl Person {
    fn make_child(&self, name: &str) -> Person {
        Person {
            name: name.to_string(),
            age: 0,
            ..self.clone()
        }
    }
}

struct Undefined;

struct Color(i32, i32, i32);

fn main() {
    let a = Person {
        surname: dbg!("Yuuki".to_string()),
        name: "Asuna".to_string(),
        age: 18,
    };

    let ch = a.make_child("Childhiko");

    dbg!(&a);

    let black = (0, 0, 0);
    let white = (16, 16, 16);
    let (r, g, b) = black;

    println!("person: {:#?} \nher child: {:?}", a, ch);
}
