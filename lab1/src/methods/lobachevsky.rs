use std::fmt;

use crate::Coefs;
use crate::AlgebraicFunc;

pub struct Lobachevsky<'a, F: AlgebraicFunc> {
    prev_coefs: Vec<f64>,
    coefs: Vec<f64>,
    iter_count: i32,
    func: &'a F
}

impl<'a, F: AlgebraicFunc> Lobachevsky<'a, F> {

    pub fn new(func: &'a F) -> Self {
        assert!(func.get_coefs().len() > 0);

        Self {
            prev_coefs: func.get_coefs().to_vec(),
            coefs:      func.get_coefs().to_vec(),
            iter_count: 0,
            func
        }
    }

    pub fn should_stop_iteration(&self, epsilon: f64) -> bool {
        self.coefs
            .iter()
            .zip(self.prev_coefs.iter())
            .map(|(cur, prev)| (1.0 - cur / (prev.powi(2))).powi(2))
            .sum::<f64>()
            .sqrt() < epsilon
    }

    pub fn calc_roots(&self) -> impl Iterator<Item = f64> + '_ {
        let two_pow_neg_iter_count = 2.0_f64.powi(-self.iter_count);
    
        self.coefs
            .iter()
            .zip(self.coefs[1..].iter())
            .map(move |(coef_prev, coef)| {
                let x = (coef_prev / coef).powf(two_pow_neg_iter_count);
                if self.func.func(x).abs() < self.func.func(-x).abs() { 
                    x
                } else {
                    -x 
                }
            })
    }

    

    pub fn next_quadration(&mut self) {
        let Self { prev_coefs, coefs, .. } = self;
        coefs
            .iter()
            .zip(prev_coefs.iter_mut())
            .enumerate()
            .for_each(|(k, (a_k, b_k))| {
                let sum: f64 = (1..(k + 1).min(coefs.len() - k))
                    .map(|j| (1 - ((j as i32 & 1) << 1)) as f64 * coefs[k - j] * coefs[k + j])
                    .sum();

                *b_k = a_k.powi(2) + 2.0 * sum; 
            });

        std::mem::swap(prev_coefs, coefs);
        self.iter_count += 1;
    }
}

impl<F> fmt::Display for Lobachevsky<'_, F>
where
    F: AlgebraicFunc
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{i}] coefs: {coefs}", i = self.iter_count, coefs = Coefs(&self.coefs))
    }

}
