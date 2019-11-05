use crate::Range as Range;
use crate::funcs::{Func, Derivative};
use super::chord::Chord;

pub struct ChordTangents<'a, F: Func + Derivative> {
    chord: Chord<'a, F>
}

impl<'a, F: Func + Derivative> ChordTangents<'a, F> {

    pub fn with_first_approach(func: &'a F, range: Range, epsilon: f64) -> Self { 
        Self { chord: Chord::with_first_approach(func, range, epsilon) }
    }

    pub fn calc_tangent_iteration(&self) -> f64 {
        let prev_approach = self.chord.get_immutable_end_value();
        let func = self.chord.get_func();

        prev_approach - (func.func(prev_approach) / func.derivative(prev_approach))
    } 

    #[inline] pub fn get_approach(&self) -> f64 {
        let range = self.get_range(); 
        (*range.start() + *range.end()) / 2.0
    }
    #[inline] pub fn get_range(&self) -> &Range { 
        self.chord.get_range()
    }
}

pub struct ChordTangentsIterationResult {
    pub chord_approach: f64,
    pub tangent_approach: f64,
    pub approach: f64
}


impl<F> Iterator for ChordTangents<'_, F>
where 
    F: Func + Derivative
{
    type Item = ChordTangentsIterationResult;

    fn next(&mut self) -> Option<Self::Item> {
        let tangent_approach = self.calc_tangent_iteration();

        self.chord.set_immutable_end(tangent_approach);
        
        self.chord.make_approach();
        
        if (tangent_approach - self.chord.get_approach()).abs() <= self.chord.get_epsilon() {
            None
        } else {
            Some(ChordTangentsIterationResult {
                chord_approach: self.chord.get_approach(),
                tangent_approach,
                approach: self.get_approach()
            })
        }

    }

}

