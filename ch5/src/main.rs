#[derive(Debug, Clone)]
#[allow(dead_code)]

struct Person {
    surname: String,
    name: String,
    age: u16, // some waifus are even 500+ years old
}

fn make_child(w: &Person, name: &str) -> Person {
    Person {
        name: name.to_string(),
        age: 0,
        ..(w.clone())
    }
}

fn main() {
    let a = Person {
        surname: "Yuuki".to_string(),
        name: "Asuna".to_string(),
        age: 18,
    };

    let ch = make_child(&a, "Childhiko");

    println!("person: {:?} \nher child: {:?}", a, ch);
}
