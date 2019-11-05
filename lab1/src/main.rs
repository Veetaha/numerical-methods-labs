use std::io::Read;

use lab1::{*, funcs::{Func}};

const EPSILON: f64 = 1e-7;
const F1_RANGES: [Range; 3] = [
    -2.0..=-1.5,
    1.2..=1.3,
    1.5..=1.55
];
const F2_RANGES: [Range; 2] = [
    0.0..=1.0,
    2.0..=4.0
];

fn main() {
    println!("Input to get results for f1");

    run_chord_tangents(F1, EPSILON, F1_RANGES.iter().cloned());

    println!("Input to get results for f2"); 

    run_chords(F2, EPSILON, F2_RANGES.iter().cloned());

}

fn run_chord_tangents<F: Func + Derivative>(f: F, epsilon: f64, ranges: impl Iterator<Item = Range>) {
    for (range, input) in ranges.zip(std::io::stdin().bytes()) {

        input.expect("EOF is unexpected, use terminal for input");

        let method = methods::ChordTangents::with_first_approach(&f, range.clone(), epsilon);

        println!(
            "[CHORD-TANGENTS] ---- first approach (chord method): for {:?} ----", 
            method.get_range()
        );

        let mut result = method.get_approach();
        let mut i = 1_usize;

        for epoch in method {
            println!(
                "[CHORD-TANGENTS] iteration {} -> {}, tangent_approach: {}, chord_approach: {}", 
                i, epoch.approach, epoch.tangent_approach, epoch.chord_approach
            );
            result = epoch.approach;
            i += 1;
        }

        println!("[CHORD-TANGENTS] Result for {:?}: {} with total of {} iterations", range, result, i);

    }
}

fn run_chords<F: Func>(f: F, epsilon: f64, ranges: impl Iterator<Item = Range>) {

    for (range, input) in ranges.zip(std::io::stdin().bytes()) {

        input.expect("EOF is unexpected, use terminal for input");

        let method = methods::Chord::with_first_approach(&f, range.clone(), epsilon);

        println!("[CHORD] ---- first approach: for {:?} ----", method.get_range());

        let mut result = method.get_approach();
        let mut i = 1_usize;

        for epoch in method {
            println!(
                "[CHORD] iteration {} -> {}, range: [{}, {}]", 
                i, epoch.approach, *epoch.range.start(), *epoch.range.end()
            );
            result = epoch.approach;
            i += 1;
        }

        println!("[CHORD] Result for {:?}: {} with total of {} iterations", range, result, i);

    }
}
