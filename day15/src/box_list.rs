use std::fmt::Debug;

use crate::lens::Lens;
use crate::lens_box::LensBox;
use crate::op::Op;

#[derive(Clone, PartialEq)]
pub struct BoxList<'a> {
    boxes: [LensBox<'a>; 256],
}

impl<'a> BoxList<'a> {
    pub fn empty() -> Self {
        Self { boxes: std::array::from_fn(|_| LensBox::empty()) }
    }

    pub fn run_op(&mut self, op: Op<'a>) {
        match op {
            Op::Eq { lens_name, number, hash } => {
                self.boxes[hash].add(Lens::new(lens_name, number));
            }
            Op::Dash { lens_name, hash } => {
                self.boxes[hash].remove(lens_name);
            }
        }
    }

    pub fn total(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(idx, lens_box)| lens_box.get_value(idx))
            .sum()
    }
}

impl Debug for BoxList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, lens_box) in self.boxes.iter().enumerate() {
            if !lens_box.is_empty() {
                writeln!(f, "Box {idx:3}: {lens_box:?}")?;
            }
        }
        Ok(())
    }
}
