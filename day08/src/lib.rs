pub mod ident;
pub mod direction;
pub mod map;
pub mod cycles;


const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";


pub mod cycle {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Cycle {
        pub offset: usize,
        pub length: usize,
    }
}


pub mod node {
    use crate::ident::Ident;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DirectionNode {
        pub left: Ident,
        pub right: Ident,
    }

    impl DirectionNode {
        pub fn new(left: Ident, right: Ident) -> Self {
            Self {
                left,
                right,
            }
        }
    }
}
