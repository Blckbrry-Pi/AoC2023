use std::ops::Range;

#[derive(Debug)]
pub struct GroupRange(Range<usize>, usize);

impl GroupRange {
    pub fn new(range: Range<usize>, len: usize) -> Self {
        Self(range, len)
    }

    pub fn range(&self) -> Range<usize> {
        self.0.clone()
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.1
    }
}

