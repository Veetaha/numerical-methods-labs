pub trait Func {
    fn func(&self, x: f64) -> f64;
}

pub trait Derivative {
    fn derivative(&self, x: f64) -> f64;
}


pub trait AlgebraicFunc: Func {
    fn get_coefs(&self) -> &[f64];
}

impl<T> Func for T where T: AlgebraicFunc {

    fn func(&self, x: f64) -> f64 {
        let mut x_i = 1_f64;
        self.get_coefs()
            .iter()
            .map(|coef| {
                let result = x_i * coef;
                x_i *= x;
                result
            })
            .sum() 
    }

}
