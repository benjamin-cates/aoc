use std::{cmp::Ordering, ops::{Deref, DerefMut}};

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

impl Deref for OrdFloat {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OrdFloat {
    fn deref_mut (&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Eq for OrdFloat { }
