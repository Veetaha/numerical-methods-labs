use std::fmt;
use std::ops;

type Range = ops::RangeInclusive<f64>;

pub struct F1 {
    body: String,
    domain_of_definition: Range
}

pub struct ChordTangentsIteration {
    range: Range,
    approach: f64
}

macro_rules! f {
    ($($params),+) => { self.call($($params),+) };
}

impl F1 {

    fn new(domain_of_definition: Range) -> Self { 
        Self { 
            body: "5 + (x**7) * (sin(x) - (x**6) * cos(x) * sqrt(PI - cos(x**3)))".to_owned(),
            domain_of_definition 
        }
    }

    /// Computes given function f1 (according to the task)
    fn call(&self, x: f64) -> f64 {
        debug_assert!(self.domain_of_definition.contains(&x));

        use std::f64::consts::PI;

        let (sin_x, cos_x) = x.sin_cos();
        let x_3 = x * x * x;
        let x_6 = x_3 * x_3;

        5.0 + (x * x_6) * (sin_x - x_6 * cos_x * (PI - x_3.cos()).sqrt())
    }

    #[inline]
    fn get_interpolated_with_fstart_fend(&self, range: Range, f_a: f64, f_b: f64) -> f64 {
        let a = *range.start();
        let b = *range.end();
        a - (f_a * (b - a)) / (f_b - a)
    }

    #[inline]
    fn get_interpolated(&self, range: Range) -> f64 {
        self.get_interpolated_with_fstart_fend(range, self.call(*range.start()), self.call(*range.end()))
    }

    fn get_root_with_chord_tangents(self) -> impl Iterator<Item = ChordTangentsIteration> {
        let range = self.domain_of_definition;

        let mut f_start  = self.call(*range.start());
        let mut f_end    = self.call(*range.end());

        debug_assert!(f_start * f_end < 0.0);

        let mut approach = self.get_interpolated_with_fstart_fend(range, f_start, f_end);

        let mut f_approach = self.call(approach);

        let (immutable_end, mutable_end) = {
            if (f_start >= 0.0) == (f_approach >= 0.0) {
                (&f_start, &f_end)
            } else {
                (&f_end, &f_start)
            }
        };

        (0..).map(|k| {
            // let range = 
            return ChordTangentsIteration {
                approach: 0.0,
                range: 1.0..=42.0
            }
        })
    }

}


impl fmt::Display for F1 {

    fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}


