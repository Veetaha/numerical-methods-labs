use std::io::Read;

use lab1::*;


fn main() {
    run_chords(f2, (&[
        0.0..=1.0,
        2.0..=4.0
    ]).iter().cloned());

    run_chords(f1, (&[
        -2.0..=-1.5,
        1.2..=1.3,
        1.5..=1.55
    ]).iter().cloned());
}


fn run_chords<F: Copy + Fn(f64) -> f64>(f: F, ranges: impl Iterator<Item = Range>) {
    let epsilon = 1e-7;

    let mut input = std::io::stdin().bytes();

    for range in ranges {
        let chord = methods::Chord::with_first_approach(f, range, epsilon);

        println!("first approach: {:?}", chord.get_range());

        for (i, epoch) in chord.enumerate() {
            println!(
                "iteration {} -> {}, range: [{}, {}]", 
                i, epoch.approach, *epoch.range.start(), *epoch.range.end()
            );

            input.next().and_then(Result::ok).unwrap();
        }

    }
}
