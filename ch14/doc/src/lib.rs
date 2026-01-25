//! Waifu
//!
//! A library for modelling waifus

pub mod waifu {
    /// Dere type. It resembles how she loves you, more info: <https://the-dere-types.fandom.com/wiki/Dere_Types_Wiki>
    pub enum Dere {
        Tsundere,
        Kuudere,
        Yandere,
        Deredere,
    }

    /// The type of how she looks. Cool, kind and cute types
    pub enum Type {
        Kawaii,
        Cool,
        Housewife,
    }

    /// Fullname of the waifu
    #[derive(Debug)]
    pub struct Fullname {
        firstname: String,
        lastname: String,
    }

    /// Waifu object
    pub struct Waifu {
        fullname: Fullname,
        look: Type,
        deretype: Dere,
        age: u32,
    }
    impl Waifu {
        pub fn confess(&self, name: &str) -> String {
            format!("{:?}: love you {name} ", &self.fullname)
        }
    }
}
