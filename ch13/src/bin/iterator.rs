trait Itereta {
    type Aitemu;

    fn nekusuto(&mut self) -> Option<Self::Aitemu>;
}

struct Shoe<'a> {
    size: u32,
    style: &'a str,
}
fn shoes_in_size<'a>(shoes: &'a Vec<Shoe<'a>>, size: u32) -> Vec<&'a Shoe<'a>> {
    shoes.into_iter().filter(|x| x.size == size).collect()
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let iterator = v1.iter();

    for i in iterator {
        println!("{i}");
    }

    let total: i32 = v1.iter().sum();

    v1.iter().map(|x| x + 1).collect::<Vec<_>>();
    println!("{v1:?}");

    let shoes = vec![
        Shoe {
            size: 82,
            style: "koja",
        },
        Shoe {
            size: 13,
            style: "importniy",
        },
        Shoe {
            size: 43,
            style: "beimportniy",
        },
    ];

    shoes_in_size(&shoes, 13);
}
