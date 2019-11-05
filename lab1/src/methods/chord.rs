use crate::Range as Range;

pub struct Chord<F: Fn(f64) -> f64> {
    func: F,
    epsilon: f64,
    range: Range,
    approach: f64,
    move_mutable_end: fn(&mut Chord<F>)
}

impl<F> Chord<F>
where F: Fn(f64) -> f64 {
    pub fn with_first_approach(func: F, range: Range, epsilon: f64) -> Self { 
        debug_assert!(range.start() < range.end());

        let f_start = func(*range.start());
        let f_end   = func(*range.end());

        debug_assert!(
            f_start * f_end < 0.0, 
            "f_start: {}, f_end: {}, start: {}, end: {},",
            f_start, f_end, *range.start(), *range.end()
        );

        let approach = Self::get_interpolated_with_fstart_fend(&range, f_start, f_end);
        let f_approach = func(approach);

        let move_mutable_end = if (f_start <= 0.0) == (f_approach <= 0.0) {
            Self::move_left_end
        } else {
            Self::move_right_end
        };


        let mut newbie = Self {
            range,
            func,
            approach,
            move_mutable_end,
            epsilon
        };
        (newbie.move_mutable_end)(&mut newbie);
        newbie
    }

    fn move_left_end(&mut self)  { self.range = self.approach..=*self.range.end(); }
    fn move_right_end(&mut self) { self.range = *self.range.start()..=self.approach; }

    #[inline] 
    pub fn get_range(&self) -> &Range { &self.range }
    #[inline] 
    pub fn get_approach(&self) -> f64 { self.approach }

    #[inline]
    fn get_interpolated_with_fstart_fend(range: &Range, f_a: f64, f_b: f64) -> f64 {
        let a = *range.start();
        let b = *range.end();
        a - (f_a * (b - a)) / (f_b - f_a)
    }

    #[inline]
    fn get_interpolated(&self) -> f64 {
        let Self { ref func, ref range, .. } = self;

        Self::get_interpolated_with_fstart_fend(range, func(*range.start()), func(*range.end()))
    }

}

pub struct ChordIterationResult {
    pub approach: f64,
    pub range: Range
}

impl<F> Iterator for Chord<F>
where 
    F: Fn(f64) -> f64 
{
    type Item = ChordIterationResult;

    fn next(&mut self) -> Option<Self::Item> {

        let prev_approach = self.approach;

        self.approach = self.get_interpolated();

        (self.move_mutable_end)(self);
        
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

