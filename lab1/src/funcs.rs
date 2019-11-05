use std::f64::consts::PI;

pub fn f1(x: f64) -> f64 {

    let (sin_x, cos_x) = x.sin_cos();
    let x_3 = x * x * x;
    let x_6 = x_3 * x_3;

    5.0 + (x * x_6) * (sin_x - x_6 * cos_x * (PI - x_3.cos()).sqrt())
}

pub fn f2(x: f64) -> f64 {
    x * (x - 2.0) + PI * ((13.0 * PI).log10() - 5.0 * x.sin())
}
