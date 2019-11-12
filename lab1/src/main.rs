use std::io::Read;

use lab1::*;

const EPSILON: f64 = 1e-7;
const F1_RANGES: [Range; 3] = [
    Range { a: -2.0, b: -1.5 },
    Range { a:  1.2, b:  1.3 },
    Range { a:  1.5, b:  1.55 }
];
const F2_RANGES: [Range; 2] = [
    Range { a: 0.0, b: 1.0 },
    Range { a: 2.0, b: 4.0 }
];

const F3_COEFS: [f64; 8] = [
    -3.0,
    -638.0,
     772.0,
     825.0,
    -589.0,
    -171.0,
     71.0,
    -2.0
];

fn main() {
    println!("Input to get results for f1");

    run_chord_tangents(F1, EPSILON, F1_RANGES.iter().cloned());

    println!("Input to get results for f2"); 

    run_fixed_point_iteration(F2, EPSILON, F2_RANGES.iter().cloned());
    run_chords(F2, EPSILON, F2_RANGES.iter().cloned());

    println!("Input to get results for f3");
    let f3 = F3::new(Vec::from(&F3_COEFS as &[f64]));
    run_lobachevsky(f3, 1e-3, EPSILON * 1e-2);
}

fn run_lobachevsky<F: AlgebraicFunc>(f: F, lob_epsilon: f64, epsilon: f64) {
    let mut method = methods::Lobachevsky::new(&f);
    let mut i = 0;
    while {
        method.next_quadration();
        i += 1;
        println!("[LOBACHEVSKY] {}", method);

        !method.should_stop_iteration(lob_epsilon) && i < 1000 // prevent infinite loops
    } {}

    let roots: Vec<_> = method.calc_roots().collect();

    println!("Lobachevsky roots approach: {:.8?}", roots);

    run_chords(f, epsilon, roots.iter().map(|root| Range { a: root - lob_epsilon, b: root + lob_epsilon }))
}

fn run_fixed_point_iteration<F: Func + Derivative>(
    f: F, 
    epsilon: f64, 
    ranges: impl Iterator<Item = Range>
) {
    for (range, _) in ranges.zip(input_char()) {

        let mut method = methods::FixedPointIteration::new(&f, range);

        println!(
            "[FIXED-POINT] ---- first approach (chord method):  {} ----", 
            method
        );

        for i in 1.. {
            let accuracy = method.next_exact();

            println!(
                "[FIXED-POINT] #{i} accuracy: {acc:.10}, {method}", 
                i = i, 
                acc = accuracy,
                method = method
            );
            
            if accuracy <= epsilon {
                println!("[FIXED-POINT] Desired accuracy was reached, root: {}", method.get_root());
                break;
            }
        }
    }
}


fn run_chord_tangents<F: Func + Derivative>(f: F, epsilon: f64, ranges: impl Iterator<Item = Range>) {
    for (range, _) in ranges.zip(input_char()) {

        let mut method = methods::ChordTangents::with_first_approach(&f, range);

        println!(
            "[CHORD-TANGENTS] ---- first approach (chord method):  {} ----", 
            method
        );

        for i in 1.. {
            let heuristic_accuracy = method.next_heuristic();

            println!(
                "[CHORD-TANGENTS] #{i} heuristic_accuracy: {acc:.10}, {method}", 
                i = i, 
                acc = heuristic_accuracy,
                method = method
            );
            
            if heuristic_accuracy <= epsilon {
                println!("[CHORD-TANGENTS] Desired accuracy was reached, root: {}", method.get_root());
                break;
            }
        }
    }
}

fn run_chords<F: Func>(f: F, epsilon: f64, ranges: impl Iterator<Item = Range>) {

    for (range, _) in ranges.zip(input_char()) {

        let mut method = methods::Chord::with_first_approach(&f, range);

        println!("[CHORD] ---- first approach: {} ----", method);

        for i in 1.. {
            let heuristic_accuracy = method.next_heuristic();
            println!(
                "[CHORD] #{i} heuristic_accuracy: {acc:.10}, {method}", 
                i = i,
                acc = heuristic_accuracy,
                method = method
            );

            if heuristic_accuracy <= epsilon {
                println!("[CHORD] Desired accuracy was reached, root: {}", method.get_root());
                break;
            }
        }
    }
}


pub fn input_char() -> impl Iterator<Item = u8> {
    std::io::stdin()
        .bytes()
        .map(|u8_result| u8_result.expect("EOF is unexpected, use terminal for input"))
}
