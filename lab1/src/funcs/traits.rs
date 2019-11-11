pub trait Func {
    fn func(&self, x: f64) -> f64;
}

pub trait Derivative {
    fn derivative(&self, x: f64) -> f64;
}


pub trait AlgebraicFunc: Func {
    fn get_coefs(&self) -> &[f64];
}
