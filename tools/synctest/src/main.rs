use chrono::Local;
use std::time::Instant;
use tempdir::TempDir;
use tpscube_core::{scramble_3x3x3_fast, History, MoveSequence, Penalty, Solve, SolveType, TimedMove};

const SOLVE_COUNT: usize = 100;

fn main() {
    let temp_dir = TempDir::new("synctest").unwrap();

    {
        let begin = Instant::now();
        let mut history = History::open_at(temp_dir.path()).unwrap();

        for i in 0..SOLVE_COUNT {
            let scramble = scramble_3x3x3_fast();
            let solution = scramble.inverse();
            let moves: Vec<TimedMove> = solution
                .iter()
                .enumerate()
                .map(|e| TimedMove::new(*e.1, e.0 as u32 * 1000))
                .collect();
            let solve = Solve {
                id: format!("{}", i),
                solve_type: SolveType::Standard3x3x3,
                session: "default".to_string(),
                scramble,
                created: Local::now(),
                time: (solution.len() * 1000) as u32,
                penalty: Penalty::None,
                device: Some("test".to_string()),
                moves: Some(moves),
            };
            history.new_solve(solve);
        }

        history.local_commit().unwrap();
        let duration = Instant::now() - begin;
        println!(
            "Added {} solves in {:.3} secs",
            SOLVE_COUNT,
            duration.as_secs_f64()
        );
    }

    {
        let begin = Instant::now();
        let history = History::open_at(temp_dir.path()).unwrap();

        let duration = Instant::now() - begin;
        println!("After restore, {} solves present", history.solves().len());
        println!(
            "Restored {} solves in {:.3} secs",
            SOLVE_COUNT,
            duration.as_secs_f64()
        );
    }
}
