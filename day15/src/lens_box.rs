use std::fmt::Debug;

use crate::lens::Lens;

#[derive(Clone, PartialEq)]
pub struct LensBox<'a> {
    labels: Vec<Lens<'a>>,
}

impl<'a> LensBox<'a> {
    pub fn empty() -> Self {
        Self { labels: vec![] }
    }
    
    pub fn new(labels: Vec<Lens<'a>>) -> Self {
        Self { labels }
    }

    pub fn get_value(&self, box_idx: usize) -> usize {
        let mut total = 0;
        for (idx, lens) in self.labels.iter().copied().enumerate() {
            total += (box_idx + 1) * (idx + 1) * lens.focal_length();
        }
        total
    }

    pub fn add(&mut self, lens: Lens<'a>) {
        let matching_label_position = self.labels.iter().position(|in_box| in_box.name() == lens.name());
        if let Some(idx) = matching_label_position {
            self.labels[idx] = lens;
        } else {
            self.labels.push(lens);
        }
    }

    pub fn remove(&mut self, label: &'a str) {
        let matching_label_position = self.labels.iter().position(|in_box| in_box.name() == label);
        if let Some(idx) = matching_label_position {
            self.labels.remove(idx);
        }
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}

impl<'a> Debug for LensBox<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for lens in &self.labels {
            write!(f, "{lens:?} ")?;
        }
        Ok(())
    }
}
