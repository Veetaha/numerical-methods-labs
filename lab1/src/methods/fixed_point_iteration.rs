use std::fmt;

use crate::range::Range;
use crate::funcs::{Func, Derivative};

pub struct FixedPointIteration<'a, F: Func + Derivative> {
    func: &'a F,
    approach: f64,
    lambda: f64,
    q: f64
}

impl<'a, F> FixedPointIteration<'a, F>
where
    F: Func + Derivative
{
    #[inline]
    fn abs_min_max(a: f64, b: f64) -> (f64, f64) {
        if a.abs() > b.abs() { (b, a) } else { (a, b) }
    }

    pub fn new(func: &'a F, Range { a, b }: Range) -> Self {
        let (alpha, gamma) = Self::abs_min_max(
            func.derivative(a),
            func.derivative(b)
        );

        let gamma_plus_alpha = alpha + gamma;

        let newbie = Self {
            lambda: 2.0 / gamma_plus_alpha,
            approach: a,
            q: (gamma - alpha) / gamma_plus_alpha,
            func
        };
        assert!(newbie.q < 1.0);
        assert!(newbie.dphi(a).abs() - newbie.q <= std::f64::EPSILON); // sanity checks
        assert!(newbie.dphi(b).abs() - newbie.q <= std::f64::EPSILON);
        newbie
    }

    #[inline]
    pub fn get_root(&self) -> f64 { self.approach }

    fn phi(&self, x: f64) -> f64 {
        x - self.lambda * self.func.func(x)
    }
    fn dphi(&self, x: f64) -> f64 {
        1.0 - self.lambda * self.func.derivative(x)
    }

    pub fn next_exact(&mut self) -> f64 {
        let prev_approach = self.approach;
        self.approach = self.phi(prev_approach);
        (self.q / (1.0 - self.q)) * (self.approach - prev_approach).abs()
    }
}

impl<F> fmt::Display for FixedPointIteration<'_, F>
where
    F: Func + Derivative 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "approach: {approach:.10}, lambda: {lambda:.10}, q: {q:.10}", 
            approach = self.approach,
            lambda   = self.lambda,
            q        = self.q
        )
    }
}

