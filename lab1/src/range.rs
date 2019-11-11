use std::fmt;

#[derive(Copy, Clone)]
pub struct Range {
    pub a: f64,
    pub b: f64
}

impl Range {

    #[inline]
    pub fn mid(&self) -> f64 {
        (self.a + self.b) / 2.0
    }

}

impl fmt::Display for Range {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.8}, {:.8}]", self.a, self.b)
    }

}
