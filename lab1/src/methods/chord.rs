mod mut_end;

use std::fmt;

use mut_end::MutEnd;
use crate::Range as Range;
use crate::funcs::{Func};


pub struct Chord<'a, F: Func> {
    func: &'a F,
    range: Range,
    mut_end: &'a MutEnd<'a, F>
}

impl<'a, F: Func> Chord<'a, F> {
    pub(super) fn get_a(&self) -> f64 { self.range.a }
    pub(super) fn get_b(&self) -> f64 { self.range.b }

    pub(super) fn set_a(&mut self, value: f64) { self.range.a = value }
    pub(super) fn set_b(&mut self, value: f64) { self.range.b = value }

    pub fn with_first_approach(func: &'a F, Range { a, b }: Range) -> Chord<'a, F> { 
        assert!(a < b);

        let f_a = func.func(a);
        let f_b = func.func(b);

        assert!(
            f_a * f_b < 0.0, 
            "Function doesn't cross OX axis at the given range: \
            f_a: {}, f_b: {}, a: {}, b: {}",
            f_a, f_b, a, b
        );

        let approach = Self::get_interpolated_with_fstart_fend(Range { a, b }, f_a, f_b);
        let f_approach = func.func(approach);

        let mut_end = if (f_a <= 0.0) == (f_approach <= 0.0) {
            &MutEnd::A
        } else {
            &MutEnd::B
        };       
        // TODO:
        // let s_ = mut_end as *const _ as usize;
        // let a_ = &MutEnd::<'a, F>::A as *const _ as usize;
        // let b_ = &MutEnd::<'a, F>::B as *const _ as usize;

        let mut newbie = Self {
            range: Range { a, b },
            func,
            mut_end
        };
        newbie.set_mut_end(approach);
        newbie
    }

    #[inline] pub fn get_range(&self) -> &Range { &self.range }
    #[inline] pub fn get_func(&self) -> &F { &self.func }

    #[inline] pub fn get_mut_end  (&self) -> f64 { (self.mut_end.get_mut)(self) }
    #[inline] pub fn get_immut_end(&self) -> f64 { (self.mut_end.get_mut)(self) }
    #[inline] pub fn set_mut_end  (&mut self, value: f64) { (self.mut_end.set_mut)(self, value) }
    #[inline] pub fn set_immut_end(&mut self, value: f64) { (self.mut_end.set_mut)(self, value) }

    #[inline] pub fn get_root(&self) -> f64 { self.range.mid() }

    pub fn make_approach(&mut self) {
        self.set_mut_end(self.get_interpolated());
    }

    #[inline]
    fn get_interpolated_with_fstart_fend(Range {a, b}: Range, f_a: f64, f_b: f64) -> f64 {
        a - (f_a * (b - a)) / (f_b - f_a)
    }

    #[inline]
    fn get_interpolated(&self) -> f64 {
        let Self { func, range, .. } = self;

        Self::get_interpolated_with_fstart_fend(
            *range, func.func(range.a), func.func(range.b)
        )
    }


    pub fn next_heuristic(&mut self) -> f64 {
        let prev_approach = self.get_mut_end();

        self.make_approach();

        (prev_approach - self.get_mut_end()).abs()
    }

}

impl<'a, F: Func> fmt::Display for Chord<'a, F> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "range: {range}, root: {root:.10}, mut_end: {mut_end}",
            range = self.range,
            root = self.get_root(),
            mut_end = self.mut_end
        )
    }

}
