use crate::ALPHABET;


#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident(usize);

impl Ident {
    pub const AAA: Self = Self(0);
    pub const ZZZ: Self = Self(25 + 25 * 36 + 25 * 36 * 36);

    pub fn new(name: &str) -> Self {
        let chars: Vec<_> = name.chars().take(3).map(|c| ALPHABET.match_indices(c).next().unwrap().0).collect();
        Self(chars[0] + chars[1] * 36 + chars[2] * 36 * 36)
    }

    pub fn last_char(self) -> char {
        ALPHABET.chars().nth(self.0 / 36 / 36 % 36).unwrap()
    }
}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letters = [self.0 % 36, self.0 / 36 % 36, self.0 / 36 / 36 % 36];
        let mut letters = letters.iter().map(|&i| ALPHABET.chars().nth(i).unwrap());
        write!(f, "{}{}{}", letters.next().unwrap(), letters.next().unwrap(), letters.next().unwrap())
    }
}
