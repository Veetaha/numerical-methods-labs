use super::AlgebraicFunc;

pub struct F3 {
    coefs: Vec<f64>
}
impl F3 {
    #[inline]
    pub fn new(coefs: Vec<f64>) -> Self {
        Self { coefs }
    }
}

impl AlgebraicFunc for F3 {
    fn get_coefs(&self) -> &[f64] { &self.coefs }
}
