use std::time::Instant;
use tpscube_core::{
    Cube, Cube2x2x2, Cube3x3x3, Cube4x4x4, InitialCubeState, SimpleSeededRandomSource,
};

fn main() {
    let mut rng = SimpleSeededRandomSource::new();
    for _ in 0..10 {
        let mut cube = Cube2x2x2::sourced_random(&mut rng);
        let start = Instant::now();
        let solution = cube.solve().unwrap();
        let end = Instant::now();

        cube.do_moves(&solution);
        assert!(cube.is_solved());

        println!(
            "2x2x2: {} ms, solution of {} moves: {:?}",
            (end - start).as_millis(),
            solution.len(),
            solution
        );
    }

    let mut rng = SimpleSeededRandomSource::new();
    let mut cube = Cube2x2x2::sourced_random(&mut rng);
    let moves = cube.solve().unwrap();

    for mv in moves {
        cube.do_move(mv);

        let start = Instant::now();
        let solution = cube.solve().unwrap();
        let end = Instant::now();

        let mut solved_cube = cube.clone();
        solved_cube.do_moves(&solution);
        assert!(solved_cube.is_solved());

        println!(
            "2x2x2: {} ms, solution of {} moves: {:?}",
            (end - start).as_millis(),
            solution.len(),
            solution
        );
    }

    let mut rng = SimpleSeededRandomSource::new();
    for _ in 0..10 {
        let mut cube = Cube3x3x3::sourced_random(&mut rng);
        let start = Instant::now();
        let solution = cube.solve().unwrap();
        let end = Instant::now();

        cube.do_moves(&solution);
        assert!(cube.is_solved());

        println!(
            "3x3x3: {} ms, solution of {} moves: {:?}",
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

        let mut solved_cube = cube.clone();
        solved_cube.do_moves(&solution);
        assert!(solved_cube.is_solved());

        println!(
            "3x3x3: {} ms, solution of {} moves: {:?}",
            (end - start).as_millis(),
            solution.len(),
            solution
        );
    }

    let mut rng = SimpleSeededRandomSource::new();
    for _ in 0..10 {
        let mut cube = Cube4x4x4::sourced_random(&mut rng);
        let start = Instant::now();
        let solution = cube.solve().unwrap();
        let end = Instant::now();

        cube.do_moves(&solution);
        assert!(cube.is_solved());

        println!(
            "4x4x4: {} ms, solution of {} moves: {:?}",
            (end - start).as_millis(),
            solution.len(),
            solution
        );
    }

    let mut rng = SimpleSeededRandomSource::new();
    let mut cube = Cube4x4x4::sourced_random(&mut rng);
    let moves = cube.solve().unwrap();

    for mv in moves {
        cube.do_move(mv);

        let start = Instant::now();
        let solution = cube.solve().unwrap();
        let end = Instant::now();

        let mut solved_cube = cube.clone();
        solved_cube.do_moves(&solution);
        assert!(solved_cube.is_solved());

        println!(
            "4x4x4: {} ms, solution of {} moves: {:?}",
            (end - start).as_millis(),
            solution.len(),
            solution
        );
    }
}
