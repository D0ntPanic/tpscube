use std::time::Instant;
use tpscube_core::{Cube, Cube3x3x3, SimpleSeededRandomSource};

fn main() {
    let mut rng = SimpleSeededRandomSource::new();
    for _ in 0..10 {
        let cube = Cube3x3x3::sourced_random(&mut rng);
        let start = Instant::now();
        let solution = cube.solve().unwrap();
        let end = Instant::now();
        println!(
            "{} ms, solution of {} moves: {:?}",
            (end - start).as_millis(),
            solution.len(),
            solution
        );
    }

    let mut rng = SimpleSeededRandomSource::new();
    let mut cube = Cube3x3x3::sourced_random(&mut rng);
    let moves = cube.solve().unwrap();

    for mv in moves {
        cube.do_move(mv);

        let start = Instant::now();
        let solution = cube.solve().unwrap();
        let end = Instant::now();
        println!(
            "{} ms, solution of {} moves: {:?}",
            (end - start).as_millis(),
            solution.len(),
            solution
        );
    }
}
