//! # Greeter
//!
//! A library for greeting.
//!

pub use self::kinds::Animal;
pub use self::kinds::Human;
pub use self::utils::talk;

pub mod kinds {
    pub trait Talk {
        fn talk(&self);
    }

    #[derive(Debug)]
    pub enum Human {
        Persian { text: String, country: String },
        English { text: String, country: String },
        Deutsche { text: String, country: String },
    }

    #[derive(Debug)]
    pub enum Animal {
        Dog { text: String, country: String },
        Cat { text: String, country: String },
    }

    impl Talk for Human {
        fn talk(&self) {
            dbg!(self);
        }
    }

    impl Talk for Animal {
        fn talk(&self) {
            dbg!(self);
        }
    }
}

pub mod utils {
    use super::kinds::*;

    /// Talks like a human/animal
    pub fn talk<T: Talk>(t: &T) {
        t.talk();
    }
}
