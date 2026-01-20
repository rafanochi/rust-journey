struct Waifu<'a> {
    name: &'a str,
    age: u16,
}

impl Waifu<'_> {
    fn can_marry(&self) -> bool {
        self.age > 18
    }
    fn confess(&self, name: &str) -> String {
        // every girl panicks before confession right?
        panic!("trying to confess... (PANICKING INSIDE)");
        format!("{}: {}, I love you", self.name, name)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    println!("HHHHHHEEEEEEREEEEEE");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }

    #[test]
    fn axaxaxa() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn can_marry() {
        let w = Waifu {
            name: "Shinobu",
            age: 548,
        };

        assert!(w.can_marry());
    }

    #[test]
    #[ignore]
    fn can_marry_asuna() {
        let w = Waifu {
            name: "Asuna",
            age: 17,
        };

        assert!(
            w.can_marry(),
            "{} CAN'T MARRY BCZ SHE IS {}!!!!!!!!!",
            w.name,
            w.age
        );
    }

    #[test]
    #[should_panic(expected = "trying to confess")]
    fn confessing() {
        let w = Waifu {
            name: "Mizuhara",
            age: 18,
        };
        w.confess("Rafa");
    }

    #[test]
    #[ignore]
    fn add_two__numbers() -> Result<(), String> {
        let result = add(2, 7);
        match result {
            10 => Ok(()),
            _ => Err(String::from("you don't even know how to ADDDD?")),
        }
    }

    #[test]
    #[ignore]
    fn hihihihi() {
        panic!("FAILLLLUURE")
    }
}
