use std::f64::consts::PI;

use super::{Func, Derivative};


pub struct F1; // Zero sized type (ZST) gets totally optimized out

impl Func for F1 {
    fn func(&self, x: f64) -> f64 {
        let (sin_x, cos_x) = x.sin_cos();
        let x_3 = x * x * x;
        let x_6 = x_3 * x_3;

        5.0 + (x * x_6) * (sin_x - x_6 * cos_x * (PI - x_3.cos()).sqrt())
    }
}

impl Derivative for F1 {
    fn derivative(&self, x: f64) -> f64 {
        let x_3 = x * x * x;
        let x_6 = x_3 * x_3;
        let x_12 = x_6 * x_6;
        let (sin_x, cos_x) = x.sin_cos();
        let (sin_x_3, cos_x_3) = x_3.sin_cos();
        let pi_minus_cos_x_3 = PI - cos_x_3;
        let x_times_cos_x = x * cos_x;

        x_6 * (7.0 * sin_x + x_times_cos_x) - x_12 * pi_minus_cos_x_3.sqrt() *
        (13.0 * cos_x - x * (sin_x - (1.5 * x * x_times_cos_x * sin_x_3) / pi_minus_cos_x_3))
    }
}
