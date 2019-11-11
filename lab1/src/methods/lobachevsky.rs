use crate::funcs::AlgebraicFunc;

pub struct Lobachevsky<'a, F: AlgebraicFunc> {
    prev_coefs: Vec<f64>,
    coefs: Vec<f64>,
    iter_index: i32,
    func: &'a F
}

impl<'a, F: AlgebraicFunc> Lobachevsky<'a, F> {
    pub fn new(coefs: Vec<f64>, func: &'a F) -> Self {
        Self {
            prev_coefs: coefs.clone(),
            coefs,
            iter_index: 0,
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
        let two_pow_iter = 2.0_f64.powi(-self.iter_index);

        self.coefs
            .iter()
            .zip(self.coefs[1..].iter())
            .map(move |(coef_prev, coef)| {
                let x = (coef / coef_prev).powf(two_pow_iter);
                if self.func.func(x).abs() < self.func.func(-x).abs() { 
                    x 
                } else { 
                    -x 
                }
            })
    }

    pub fn next_quadration(&mut self) {
        let next_coefs = &mut self.prev_coefs;
        for i in 0..self.coefs.len() {
            let mut a = 0.0;
            let mut offset = 0_usize;
            while i - offset >= 0 &&
                  i + offset < self.coefs.len() 
            {
                a = a + (-1_f64).powi(offset as i32) * 
                    (if offset == 0 { 1.0 } else { 2.0 }) *
                    self.coefs[i - offset] * self.coefs[i + offset];

                offset += 1;
            }
            next_coefs[i] = a;
        }
        std::mem::swap(next_coefs, &mut self.coefs);
        self.iter_index += 1;
    }


}
