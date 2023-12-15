use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Lens<'a> {
    pub name: &'a str,
    pub focal_length: usize,
}

impl Debug for Lens<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.name, self.focal_length)
    }
}


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
            total += (box_idx + 1) * (idx + 1) * lens.focal_length;
        }
        total
    }

    pub fn add(&mut self, lens: Lens<'a>) {
        if let Some(idx) = self.labels.iter().position(|lens_to_check| lens_to_check.name == lens.name) {
            self.labels[idx] = lens;
        } else {
            self.labels.push(lens);
        }
    }

    pub fn remove(&mut self, label: &'a str) {
        if let Some(idx) = self.labels.iter().position(|lens| lens.name == label) {
            self.labels.remove(idx);
        }
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
