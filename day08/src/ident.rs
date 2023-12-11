use crate::ALPHABET;


const MULT: usize = ALPHABET.len();
const Z_IDX: usize = 25;



#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident(usize);

impl Ident {
    pub const AAA: Self = Self(0);
    pub const ZZZ: Self = Self(Z_IDX + Z_IDX * MULT + Z_IDX * MULT * MULT);

    pub fn new(name: &str) -> Self {
        let chars: Vec<_> = name.chars().take(3).map(|c| ALPHABET.match_indices(c).next().unwrap().0).collect();

        Self(chars[0] + chars[1] * MULT + chars[2] * MULT * MULT)
    }

    pub fn last_char(&self) -> char {
        ALPHABET.chars().nth(self.get_2()).unwrap()
    }


    pub fn end_a(&self) -> bool { self.last_char() == 'A' }
    pub fn end_z(&self) -> bool { self.last_char() == 'Z' }
    

    pub fn get_0(&self) -> usize { self.0 % MULT }
    pub fn get_1(&self) -> usize { self.0 / MULT % MULT }
    pub fn get_2(&self) -> usize { self.0 / MULT / MULT % MULT }
}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letters = [
            ALPHABET.chars().nth(self.get_0()).unwrap(),
            ALPHABET.chars().nth(self.get_1()).unwrap(),
            ALPHABET.chars().nth(self.get_2()).unwrap(),
        ];
        write!(f, "{}{}{}", letters[0], letters[1], letters[2])
    }
}
