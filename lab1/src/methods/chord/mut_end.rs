
use std::fmt;

use crate::Func;
use super::Chord;

#[derive(Clone, Copy)]
pub(super) struct MutEnd<'a, F: Func> { 
    pub(super) get_mut:   fn(chord: &Chord<'a, F>) -> f64,
    pub(super) get_immut: fn(chord: &Chord<'a, F>) -> f64,
    pub(super) set_mut:   fn(chord: &mut Chord<'a, F>, value: f64),
    pub(super) set_immut: fn(chord: &mut Chord<'a, F>, value: f64),
}
impl<'a, F> MutEnd<'a, F>
where
    F: Func + 'a
{
    pub(super) const A: Self = Self {
        get_mut:   Chord::get_a,
        set_mut:   Chord::set_a,
        get_immut: Chord::get_b,
        set_immut: Chord::set_b
    };
    pub(super) const B: Self = Self {
        get_mut:   Chord::get_b,
        set_mut:   Chord::set_b,
        get_immut: Chord::get_a,
        set_immut: Chord::set_a
    };
}

impl<F: Func> fmt::Display for MutEnd<'_, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO:
        // let s = self as *const _ as usize;
        // let a = &Self::A as *const _ as usize;
        // let b = &Self::B as *const _ as usize;
        write!(f, "{}", if self.get_mut as usize == Self::A.get_mut as usize {
            "a"
        } else {
            assert!(self.get_mut as usize == Self::B.get_mut as usize);
            "b"
        })
    }
}
