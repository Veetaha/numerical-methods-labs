use std::fmt;

use crate::Range as Range;
use crate::funcs::{Func, Derivative};
use super::chord::Chord;

pub struct ChordTangents<'a, F: Func + Derivative> {
    chord: Chord<'a, F>
}

pub struct ChordTangentsHeuristicIterationResult {
    pub heuristic_accuracy: f64
}

impl<'a, F: Func + Derivative> ChordTangents<'a, F> {

    pub fn with_first_approach(func: &'a F, range: Range) -> Self { 
        Self { chord: Chord::with_first_approach(func, range) }
    }

    pub fn calc_tangent_iteration(&self) -> f64 {
        let prev_approach = self.chord.get_immut_end();
        let func = self.chord.get_func();

        prev_approach - (func.func(prev_approach) / func.derivative(prev_approach))
    }

    #[inline] pub fn get_root(&self) -> f64 { self.chord.get_root() }
    // #[inline] pub fn get_tangent_approach(&self) -> f64 { self.chord.get_immut_end() }
    // #[inline] pub fn get_chord_approach  (&self) -> f64 { self.chord.get_mut_end() }

    #[inline] pub fn get_range(&self) -> &Range { self.chord.get_range() }

    pub fn next_heuristic(&mut self) -> f64 {
        let tangent_approach = self.calc_tangent_iteration();

        self.chord.set_immut_end(tangent_approach);

        self.chord.make_approach();

        (tangent_approach - self.chord.get_mut_end()).abs()
    }

}


impl<'a, F> fmt::Display for ChordTangents<'a, F>
where
    F: Func + Derivative
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chord)
    }
}
