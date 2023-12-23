use std::fmt::Debug;

type InternalInt = u32;
type AtomicInt = std::sync::atomic::AtomicU32;
const HEX_FORMATTING_WIDTH: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(InternalInt);

impl Id {
    pub fn new_unique() -> Self {
        static NEXT_ID: AtomicInt = AtomicInt::new(0);
        Self(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<0x{:0HEX_FORMATTING_WIDTH$x}>", self.0)
    }
}
