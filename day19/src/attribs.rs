#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribSplit {
    pub applies: Option<AttribRange>,
    pub doesnt_apply: Option<AttribRange>,
}

impl AttribRange {
    pub fn new(
        x: (usize, usize),
        m: (usize, usize),
        a: (usize, usize),
        s: (usize, usize),
    ) -> Self {
        Self { x, m, a, s }
    }

    pub fn access(self, attrib: Attrib) -> (usize, usize) {
        match attrib {
            Attrib::X => self.x,
            Attrib::M => self.m,
            Attrib::A => self.a,
            Attrib::S => self.s,
        }
    }

    pub fn len(self) -> usize {
        let x_range = self.x.1 - self.x.0;
        let m_range = self.m.1 - self.m.0;
        let a_range = self.a.1 - self.a.0;
        let s_range = self.s.1 - self.s.0;
        x_range * m_range * a_range * s_range
    }
    pub fn is_empty(self) -> bool {
        let x_empty = self.x.1 == self.x.0;
        let m_empty = self.m.1 == self.m.0;
        let a_empty = self.a.1 == self.a.0;
        let s_empty = self.s.1 == self.s.0;

        x_empty || m_empty || a_empty || s_empty
    }

    fn normalize(self) -> Self {
        Self {
            x: (self.x.0, self.x.1.max(self.x.0)),
            m: (self.m.0, self.m.1.max(self.m.0)),
            a: (self.a.0, self.a.1.max(self.a.0)),
            s: (self.s.0, self.s.1.max(self.s.0)),
        }
    }

    pub fn with_attrib_range(self, attrib: Attrib, range: (usize, usize)) -> Self {
        match attrib {
            Attrib::X => Self { x: range, ..self },
            Attrib::M => Self { m: range, ..self },
            Attrib::A => Self { a: range, ..self },
            Attrib::S => Self { s: range, ..self },
        }.normalize()
    }

    pub fn split_gt(self, attrib: Attrib, value: usize) -> AttribSplit {
        let applicable_value = self.access(attrib);

        let applies = self.with_attrib_range(attrib, (value + 1, applicable_value.1));
        let doesnt_apply = self.with_attrib_range(attrib, (applicable_value.0, value + 1));

        AttribSplit {
            applies: (!applies.is_empty()).then_some(applies),
            doesnt_apply: (!doesnt_apply.is_empty()).then_some(doesnt_apply),
        }
    }

    pub fn split_lt(self, attrib: Attrib, value: usize) -> AttribSplit {
        let applicable_value = self.access(attrib);

        let applies = self.with_attrib_range(attrib, (applicable_value.0, value));
        let doesnt_apply = self.with_attrib_range(attrib, (value, applicable_value.1));

        AttribSplit {
            applies: (!applies.is_empty()).then_some(applies),
            doesnt_apply: (!doesnt_apply.is_empty()).then_some(doesnt_apply),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attribs {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Attribs {
    pub fn parse(line: &str) -> Self {
        let inner = line.trim_matches(['{', '}']);

        let parts: Vec<_> = inner.split(',').collect();

        let [x, m, a, s] = [
            parts[0].trim_start_matches("x="),
            parts[1].trim_start_matches("m="),
            parts[2].trim_start_matches("a="),
            parts[3].trim_start_matches("s="),
        ];

        Self {
            x: x.parse().unwrap(),
            m: m.parse().unwrap(),
            a: a.parse().unwrap(),
            s: s.parse().unwrap(),
        }
    }

    pub fn access(&self, attrib: Attrib) -> usize {
        match attrib {
            Attrib::X => self.x,
            Attrib::M => self.m,
            Attrib::A => self.a,
            Attrib::S => self.s,
        }
    }

    pub fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attrib { X, M, A, S }

impl Attrib {
    pub fn parse(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("Invalid attrib: {}", c),
        }
    }

    pub fn access(&self, attribs: Attribs) -> usize {
        match self {
            Self::X => attribs.x,
            Self::M => attribs.m,
            Self::A => attribs.a,
            Self::S => attribs.s,
        }
    }
}


