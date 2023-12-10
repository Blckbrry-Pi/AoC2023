#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sequence {
    AllZeros,
    Constant(isize),
    List(Vec<isize>),
}

impl Sequence {
    pub fn new(numbers: &[isize]) -> Self {
        if numbers.is_empty() || numbers.iter().all(|&n| n == 0) {
            Self::AllZeros
        } else if numbers.iter().all(|&n| n == numbers[0]) {
            Self::Constant(numbers[0])
        } else {
            Self::List(numbers.to_vec())
        }
    }
    pub fn parse(line: &str) -> Self {
        let numbers: Vec<_> = line.split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        Self::new(&numbers)
    }

    pub fn deriv(&self) -> Self {
        match self {
            Self::AllZeros => Self::AllZeros,
            Self::Constant(_) => Self::AllZeros,
            Self::List(numbers) => {
                let mut deriv = Vec::with_capacity(numbers.len() - 1);
                for (prev, next) in numbers.iter().copied().zip(numbers.iter().copied().skip(1)) {
                    deriv.push(next - prev);
                }
                Self::new(&deriv)
            }
        }
    }

    pub fn integ(&self, start: isize, len: usize, steps_back: usize) -> Self {
        match self {
            Self::AllZeros => if start == 0 {
                Self::AllZeros
            } else {
                Self::Constant(start)
            },
            Self::Constant(n) => {
                let mut integ = Vec::with_capacity(len);
                let mut curr = start - (steps_back as isize * n);
                for _ in 0..len {
                    integ.push(curr);
                    curr += n;
                }
                Self::new(&integ)
            },
            Self::List(numbers) => {
                let mut integ = Vec::with_capacity(numbers.len());
                let mut curr = start - numbers.iter().copied().take(steps_back).sum::<isize>();
                for &n in numbers {
                    integ.push(curr);
                    curr += n;
                }
                integ.push(curr);
                Self::new(&integ)
            }
        }
    }

    pub fn extrapolate(self) -> Self {
        match &self {
            Self::AllZeros => Self::AllZeros,
            Self::Constant(n) => Self::Constant(*n),
            Self::List(numbers) => {
                self.deriv().extrapolate().integ(numbers[0], numbers.len()+1, 0)
            }
        }
    }

    pub fn extrapolate_back(self) -> Self {
        match &self {
            Self::AllZeros => Self::AllZeros,
            Self::Constant(n) => Self::Constant(*n),
            Self::List(numbers) => {
                self.deriv().extrapolate_back().integ(numbers[0], numbers.len()+1, 1)
            }
        }
    }

    pub fn last(self) -> Option<isize> {
        match self {
            Self::AllZeros => Some(0),
            Self::Constant(n) => Some(n),
            Self::List(numbers) => numbers.last().copied(),
        }
    }
    pub fn first(self) -> Option<isize> {
        match self {
            Self::AllZeros => Some(0),
            Self::Constant(n) => Some(n),
            Self::List(numbers) => numbers.first().copied(),
        }
    }
}

impl std::ops::Index<usize> for Sequence {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::AllZeros => &0,
            Self::Constant(n) => n,
            Self::List(numbers) => &numbers[index],
        }
    }
}
