use std::f64::consts::PI;

use super::{Func};

pub struct F2;

impl Func for F2 {

    #[inline]
    fn func(&self, x: f64) -> f64 {
        x * (x - 2.0) + PI * ((13.0 * PI).log10() - 5.0 * x.sin())
    }
}
