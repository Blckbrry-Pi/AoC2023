#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hasher {
    curr_val: usize,
}

impl Hasher {
    pub fn new(starting_value: usize) -> Self {
        Self { curr_val: starting_value }
    }

    pub fn hash_char(&mut self, c: char) {
        let ascii_value = c as usize;
        self.curr_val += ascii_value;
        self.curr_val *= 17;

        self.curr_val %= 256;
    }

    pub fn hash_str(&mut self, s: &str) {
        for c in s.chars() {
            self.hash_char(c);
        }
    }

    pub fn get_val(&self) -> usize {
        self.curr_val
    }

    pub fn hash_val_of_str(s: &str) -> usize {
        let mut hasher = Self::default();
        hasher.hash_str(s);
        hasher.get_val()
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new(0)
    }
}

