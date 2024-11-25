use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct OrdFloat(f64);


impl From<f64> for OrdFloat {
    fn from(value: f64) -> Self {
        OrdFloat(value)
    }
}

impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for OrdFloat { }
