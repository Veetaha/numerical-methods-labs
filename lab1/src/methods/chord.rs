use crate::Range as Range;
use crate::funcs::{Func};

#[derive(Clone, Copy)]
pub enum MutableEnd { Start, End }

pub struct Chord<'a, F: Func> {
    func: &'a F,
    epsilon: f64,
    range: Range,
    approach: f64,
    mutable_end: MutableEnd
}

impl<'a, F: Func> Chord<'a, F> {
    pub fn with_first_approach(func: &'a F, range: Range, epsilon: f64) -> Chord<'a, F> { 
        debug_assert!(range.start() < range.end());

        let f_start = func.func(*range.start());
        let f_end   = func.func(*range.end());

        debug_assert!(
            f_start * f_end < 0.0, 
            "f_start: {}, f_end: {}, start: {}, end: {},",
            f_start, f_end, *range.start(), *range.end()
        );

        let approach = Self::get_interpolated_with_fstart_fend(&range, f_start, f_end);
        let f_approach = func.func(approach);

        let mutable_end = if (f_start <= 0.0) == (f_approach <= 0.0) {
            MutableEnd::Start
        } else {
            MutableEnd::End
        };


        let mut newbie = Self {
            range,
            func,
            approach,
            mutable_end,
            epsilon
        };
        newbie.set_mutable_end(approach);
        newbie
    }

    #[inline] pub fn get_range(&self) -> &Range { &self.range }
    #[inline] pub fn get_func(&self) -> &F { &self.func }
    #[inline] pub fn get_epsilon(&self) -> f64 { self.epsilon }
    #[inline] pub fn get_approach(&self) -> f64 { self.approach }

    #[inline] pub fn get_immutable_end_value(&self) -> f64 { 
        match self.mutable_end {
            MutableEnd::Start => *self.range.end(),
            MutableEnd::End   => *self.range.start()
        }
    }

    pub fn make_approach(&mut self) {
        self.approach = self.get_interpolated();

        self.set_mutable_end(self.approach);
    }

    #[inline]
    pub(super) fn set_mutable_end(&mut self, value: f64) { 
        self.range = match self.mutable_end {
            MutableEnd::Start => value..=*self.range.end(),
            MutableEnd::End   => *self.range.start()..=value
        };
    }

    #[inline]
    pub(super) fn set_immutable_end(&mut self, value: f64) {
        self.range = match self.mutable_end {
            MutableEnd::Start => *self.range.start()..=value,
            MutableEnd::End   => value..=*self.range.end()
        };
    }

    #[inline]
    fn get_interpolated_with_fstart_fend(range: &Range, f_a: f64, f_b: f64) -> f64 {
        let a = *range.start();
        let b = *range.end();
        a - (f_a * (b - a)) / (f_b - f_a)
    }

    #[inline]
    fn get_interpolated(&self) -> f64 {
        let Self { ref func, ref range, .. } = self;

        Self::get_interpolated_with_fstart_fend(
            range, func.func(*range.start()), func.func(*range.end())
        )
    }

}

pub struct ChordIterationResult {
    pub approach: f64,
    pub range: Range
}

impl<F: Func> Iterator for Chord<'_, F> {
    type Item = ChordIterationResult;

    fn next(&mut self) -> Option<Self::Item> {

        let prev_approach = self.approach;

        self.make_approach();
        
        if (prev_approach - self.approach).abs() <= self.epsilon {
            None
        } else {
            Some(ChordIterationResult {
                approach: self.approach,
                range: self.range.clone()
            })
        }
    }

}

