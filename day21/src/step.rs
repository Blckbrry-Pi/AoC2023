#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Step {
    NN,
    NE,
    EE,
    SE,
    SS,
    SW,
    WW,
    NW,
}

impl Step {
    pub fn orthogonal() -> impl Iterator<Item = Step> {
        [Step::NN, Step::EE, Step::SS, Step::WW].into_iter()
    }

    pub fn diagonal() -> impl Iterator<Item = Step> {
        [Step::NE, Step::SE, Step::SW, Step::NW].into_iter()
    }
}
