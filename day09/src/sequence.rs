#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sequence {
    AllZeros,
    Constant(isize),
    List(Vec<isize>),
}

impl Sequence {
    /// Creates a sequence from a list of numbers.
    /// 
    /// By contract, the list is expected to have at least 1 item.
    /// Otherwise, this function will panic.
    pub fn new(numbers: &[isize]) -> Self {
        let is_constant = numbers.iter().all(|&n| n == numbers[0]);
        let is_all_zeros = is_constant && numbers[0] == 0;

        if is_all_zeros {
            Self::AllZeros
        } else if is_constant {
            Self::Constant(numbers[0])
        } else {
            Self::List(numbers.to_vec())
        }
    }

    /// This line parses a sequence from a string.
    /// 
    /// The string should be of the format `<number> [<number> ...]`
    /// 
    /// Anything else is liable to cause a panic.
    pub fn parse(line: &str) -> Self {
        let numbers: Vec<_> = line.split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        Self::new(&numbers)
    }

    /// Gets a derivative-like sequence from this sequence.
    /// 
    /// Returned is a sequence of differences between the values of this
    /// sequence.
    pub fn deriv(&self) -> Self {
        match self {
            Self::AllZeros => Self::AllZeros,
            Self::Constant(_) => Self::AllZeros,
            Self::List(numbers) => {
                let mut deriv = Vec::with_capacity(numbers.len() - 1);

                let pairs = numbers.iter().copied().zip(numbers.iter().copied().skip(1));
                for (curr, next) in pairs {
                    deriv.push(next - curr);
                }

                Self::new(&deriv)
            }
        }
    }

    /// This gets an integral-like sequence from this sequence.
    /// 
    /// Returned is a sequence of partial sums of this sequence.
    /// 
    /// The `start` parameter is the starting value of the sequence.
    /// 
    /// The `len` parameter is the length of the sequence. Note that this
    /// parameter only affects the length of the returned sequence if it is a
    /// constant or all-zero sequence.
    /// 
    /// `steps_back` is the number of steps it should take to reach the `start`
    /// value in the sequence. This is useful for extrapolating the sequence
    /// backwards.
    pub fn integ(&self, start: isize, len: usize, steps_back: usize) -> Self {
        match self {
            Self::AllZeros => if start == 0 {
                Self::AllZeros
            } else {
                Self::Constant(start)
            },

            Self::Constant(n) => {
                let mut integ_numbers = Vec::with_capacity(len);

                let true_start = start - (steps_back as isize * n);
                let mut curr = true_start;
                for _ in 0..len {
                    integ_numbers.push(curr);
                    curr += n;
                }

                Self::new(&integ_numbers)
            },

            Self::List(numbers) => {
                let mut integ_numbers = Vec::with_capacity(numbers.len() + 1);

                let sum_before = numbers.iter().copied().take(steps_back).sum::<isize>();
                let mut curr = start - sum_before;
                for &n in numbers {
                    integ_numbers.push(curr);
                    curr += n;
                }
                integ_numbers.push(curr);

                Self::new(&integ_numbers)
            }
        }
    }

    /// This gets a sequence with the next number predicted by a combinations of
    /// recursion and applications of `deriv` and `integ`.
    pub fn extrapolate(self) -> Self {
        match &self {
            Self::AllZeros => Self::AllZeros,
            Self::Constant(n) => Self::Constant(*n),
            Self::List(numbers) => {
                let start = numbers[0];
                let final_len = numbers.len() + 1;

                self.deriv()
                    .extrapolate()
                    .integ(start, final_len, 0)
            }
        }
    }

    /// This gets a sequence with the previous number predicted by a combinations of
    /// recursion and applications of [`deriv`][`Sequence::deriv()`] and [`integ`][`Sequence::integ()`].
    pub fn extrapolate_back(self) -> Self {
        match &self {
            Self::AllZeros => Self::AllZeros,
            Self::Constant(n) => Self::Constant(*n),
            Self::List(numbers) => {
                let start = numbers[0];
                let final_len = numbers.len() + 1;
                
                self.deriv()
                    .extrapolate_back()
                    .integ(start, final_len, 1)
            }
        }
    }

    /// Gets the last value in the sequence.
    /// 
    /// Use in conjunction with [`extrapolate`][`Sequence::extrapolate()`] to get
    /// the predicted next value in the sequence.
    pub fn last(self) -> Option<isize> {
        match self {
            Self::AllZeros => Some(0),
            Self::Constant(n) => Some(n),
            Self::List(numbers) => numbers.last().copied(),
        }
    }

    /// Gets the first value in the sequence.
    /// 
    /// Use in conjunction with
    /// [`extrapolate_back`][`Sequence::extrapolate_back()`] to get the predicted
    /// previous value in the sequence.
    pub fn first(self) -> Option<isize> {
        match self {
            Self::AllZeros => Some(0),
            Self::Constant(n) => Some(n),
            Self::List(numbers) => numbers.first().copied(),
        }
    }
}


// Implementing `Index` makes writing [`extrapolate`][`Sequence::extrapolate()`]
// and [`extrapolate_back`][`Sequence::extrapolate_back()`] much easier.
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
