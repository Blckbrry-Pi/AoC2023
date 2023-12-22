use std::hash::Hasher;
use std::hash::BuildHasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NoHashHasher(u64);

impl Hasher for NoHashHasher {
    fn write(&mut self, bytes: &[u8]) {
        for group_of_8 in bytes.chunks(8) {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(group_of_8);
            let value = u64::from_le_bytes(bytes);
            self.0 = self.0.wrapping_add(value);
        }
    }

    fn write_usize(&mut self, i: usize) { self.0 = self.0.wrapping_add(i as u64); }
    fn write_isize(&mut self, i: isize) { self.0 = self.0.wrapping_add(i as u64); }

    fn finish(&self) -> u64 {
        self.0
    }
}

impl BuildHasher for NoHashHasher {
    type Hasher = NoHashHasher;

    fn build_hasher(&self) -> Self::Hasher {
        *self
    }
}

pub type HashMap<T, E> = std::collections::HashMap<T, E, NoHashHasher>;
pub type HashSet<T> = std::collections::HashSet<T, NoHashHasher>;
