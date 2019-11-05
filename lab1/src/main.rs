use std::io::Read;

use lab1::*;


fn main() {
    println!("Input to get results for f1");

    run_chords(f2, (&[
        0.0..=1.0,
        2.0..=4.0
    ]).iter().cloned());

    println!("Input to get results for f2");

    run_chords(f1, (&[
        -2.0..=-1.5,
        1.2..=1.3,
        1.5..=1.55
    ]).iter().cloned());
}


fn run_chords<F: Copy + Fn(f64) -> f64>(f: F, ranges: impl Iterator<Item = Range>) {
    let epsilon = 1e-7;


    for (range, input) in ranges.zip(std::io::stdin().bytes()) {

        input.expect("EOF is unexpected, use terminal for input");

        let chord = methods::Chord::with_first_approach(f, range.clone(), epsilon);

        println!("first approach: for {:?}", chord.get_range());

        let mut result = chord.get_approach();
        let mut i = 1_usize;

        for epoch in chord {
            println!(
                "iteration {} -> {}, range: [{}, {}]", 
                i, epoch.approach, *epoch.range.start(), *epoch.range.end()
            );
            result = epoch.approach;
            i += 1;
        }

        println!("Result for {:?}: {} with total of {} iterations", range, result, i);

    }
}
