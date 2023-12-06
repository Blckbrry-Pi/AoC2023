#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdRange {
    pub min: usize,
    pub max: usize,
}

impl IdRange {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            min,
            max: max.max(min),
        }
    }
    pub fn new_start_len(start: usize, len: usize) -> Self { Self::new(start, start + len) }

    pub fn contains(&self, id: usize) -> bool {
        id >= self.min && id < self.max
    }
    
    pub fn is_empty(self) -> bool { self.min >= self.max }
    pub fn len(self) -> usize { self.max - self.min }

    pub fn intersect(self, other: Self) -> Self {
        Self::new(self.min.max(other.min), self.max.min(other.max))
    }

    pub fn union(self, other: Self) -> impl Iterator<Item = Self> {
        if self.max < other.min || self.min > other.max {
            [Some(self), Some(other)]
        } else {
            [Some(Self::new(self.min.min(other.min), self.max.max(other.max))), None]
        }.into_iter().flatten()
    }

    pub fn diff(self, other: Self) -> impl Iterator<Item = Self> {
        // Outside
        if self.max <= other.min || self.min >= other.max {
            [Some(self), None]
        
        // Inside
        } else if self.min < other.min && other.max < self.max {
            [Some(Self::new(self.min, other.min)), Some(Self::new(other.max, self.max))]
        
        // Left
        } else if self.max > other.max {
            [Some(Self::new(other.max, self.max)), None]

        // Right
        } else if self.min < other.min {
            [Some(Self::new(self.min, other.min)), None]
        } else {
            [None, None]
        }.into_iter().flatten()
    }
}

impl Ord for IdRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.min.cmp(&other.min) {
            std::cmp::Ordering::Equal => self.max.cmp(&other.max),
            x => x,
        }
    }
}

impl PartialOrd for IdRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangeMap(IdRange, IdRange);

impl RangeMap {
    pub fn parse(line: &str) -> Self {
        let mut parts = line.split(' ');

        let dst = parts.next().unwrap();
        let src = parts.next().unwrap();
        let len = parts.next().unwrap();

        let dst = IdRange::new_start_len(dst.parse().unwrap(), len.parse().unwrap());
        let src = IdRange::new_start_len(src.parse().unwrap(), len.parse().unwrap());

        Self(src, dst)
    }

    pub fn new(from: IdRange, to: IdRange) -> Self {
        Self(from, to)
    }

    pub fn contains(&self, id: usize) -> bool {
        self.0.contains(id)
    }

    pub fn map(&self, id: usize) -> Option<usize> {
        if self.0.contains(id) {
            Some(id - self.0.min + self.1.min)
        } else {
            None
        }
    }

    pub fn map_default(&self, id: usize) -> usize {
        self.map(id).unwrap_or(id)
    }

    pub fn map_range(&self, range: IdRange) -> IdRange {
        if range.intersect(self.0).is_empty() {
            IdRange::new(0, 0)
        } else {
            IdRange::new(
                self.map_default(range.intersect(self.0).min),
                self.map_default(range.intersect(self.0).max - 1) + 1,
            )
        }
    }

    pub fn unmapped_range(&self, range: IdRange) -> impl Iterator<Item = IdRange> {
        range.diff(self.0)
    }
}

pub fn union_list(ranges: &[IdRange]) -> Vec<IdRange> {
    let mut ranges: Vec<IdRange> = ranges.iter().cloned().filter(|range| !range.is_empty()).collect();

    #[allow(clippy::never_loop)]
    'outside: {
        loop {
            'continuer: {
                for i in 0..ranges.len() {
                    for j in (i+1)..ranges.len() {
                        let range_a = ranges[i];
                        let range_b = ranges[j];
        
                        let union_ranges: Vec<IdRange> = range_a.union(range_b).collect();
                        if union_ranges.len() != 2 {
                            ranges = ranges
                                .into_iter()
                                .filter(|range| *range != range_a && *range != range_b)
                                .chain(union_ranges)
                                .collect();
                            break 'continuer;
                        }
                    }
                }
                break 'outside;
            }
        }
    }
    ranges
}
