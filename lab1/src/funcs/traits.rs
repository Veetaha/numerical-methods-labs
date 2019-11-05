pub trait Func {
    fn func(&self, x: f64) -> f64;
}

pub trait Derivative {
    fn derivative(&self, x: f64) -> f64;
}
