use crate::hasher::Hasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op<'a> {
    Eq {
        lens_name: &'a str,
        number: usize,
        hash: usize,
    },
    Dash {
        lens_name: &'a str,
        hash: usize,
    },
}

impl<'a> From<&'a str> for Op<'a> {
    fn from(value: &'a str) -> Self {

        let is_eq = value.contains('=');
        if is_eq {
            let mut split = value.split('=');
            let lens_name = split.next().unwrap();
            let number = split.next().unwrap().parse::<usize>().unwrap();
            let hash = Hasher::hash_val_of_str(lens_name);

            Self::Eq { lens_name, number, hash }
        } else {
            let lens_name = value.trim_end_matches('-');
            let hash = Hasher::hash_val_of_str(lens_name);
            Self::Dash { lens_name, hash }
        }
    }
}
