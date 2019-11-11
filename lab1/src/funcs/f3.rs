use super::{Func, AlgebraicFunc};

pub struct F3 {
    coefs: Vec<f64>
}

impl Func for F3 {
    fn func(&self, mut x: f64) -> f64 {
        let x_i: f64 = 1.0;
        self.coefs
            .iter()
            .enumerate()
            .map(|(i, coef)| {
                let result = x_i * coef;
                x_i *= x;
                result
            })
            .sum() 
    }
}
impl AlgebraicFunc for F3 {
    fn get_coefs(&self) -> &[f64] { &self.coefs }
}
