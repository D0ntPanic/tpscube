#![feature(box_syntax)]

use std::boxed::Box;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::vec::Vec;

use tpscube_core::{Cube, Cube3x3x3, Move};

struct Progress {
    filled: usize,
    total: usize,
}

impl Progress {
    fn complete(&self) -> bool {
        self.filled == self.total
    }
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} / {}", self.filled, self.total)
    }
}

struct TableGenerator {
    corner_orientation_move_table:
        Box<[[Option<u16>; Move::count_3x3x3()]; Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT]>,
    corner_permutation_move_table:
        Box<[[Option<u16>; Move::count_3x3x3()]; Cube3x3x3::CORNER_PERMUTATION_INDEX_COUNT]>,
    edge_orientation_move_table:
        Box<[[Option<u16>; Move::count_3x3x3()]; Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT]>,
    equatorial_edge_slice_move_table:
        Box<[[Option<u16>; Move::count_3x3x3()]; Cube3x3x3::EDGE_SLICE_INDEX_COUNT]>,
    phase_2_edge_permutation_move_table:
        Box<[[Option<u16>; Move::count_3x3x3()]; Cube3x3x3::PHASE_2_EDGE_PERMUTATION_INDEX_COUNT]>,
    phase_2_equatorial_edge_permutation_move_table: Box<
        [[Option<u16>; Move::count_3x3x3()];
            Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT],
    >,
    corner_orientation_edge_slice_prune_table: Box<
        [[Option<u8>; Cube3x3x3::EDGE_SLICE_INDEX_COUNT];
            Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT],
    >,
    edge_orientation_prune_table: Box<
        [[Option<u8>; Cube3x3x3::EDGE_SLICE_INDEX_COUNT]; Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT],
    >,
    combined_orientation_prune_table: Box<
        [[Option<u8>; Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT];
            Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT],
    >,
    corner_edge_permutation_prune_table: Box<
        [[Option<u8>; Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];
            Cube3x3x3::CORNER_PERMUTATION_INDEX_COUNT],
    >,
    phase_2_edge_permutation_prune_table: Box<
        [[Option<u8>; Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];
            Cube3x3x3::PHASE_2_EDGE_PERMUTATION_INDEX_COUNT],
    >,
    corner_orientation_prune_table: Box<[Option<u8>; Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT]>,
    corner_permutation_prune_table: Box<[Option<u8>; Cube3x3x3::CORNER_PERMUTATION_INDEX_COUNT]>,
}

impl TableGenerator {
    fn new() -> Self {
        let mut tables = TableGenerator {
            corner_orientation_move_table: box [[None; Move::count_3x3x3()];
                Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT],
            corner_permutation_move_table: box [[None; Move::count_3x3x3()];
                Cube3x3x3::CORNER_PERMUTATION_INDEX_COUNT],
            edge_orientation_move_table: box [[None; Move::count_3x3x3()];
                Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT],
            equatorial_edge_slice_move_table: box [[None; Move::count_3x3x3()];
                Cube3x3x3::EDGE_SLICE_INDEX_COUNT],
            phase_2_edge_permutation_move_table: box [[None; Move::count_3x3x3()];
                Cube3x3x3::PHASE_2_EDGE_PERMUTATION_INDEX_COUNT],
            phase_2_equatorial_edge_permutation_move_table: box [[None; Move::count_3x3x3()];
                Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT],
            corner_orientation_edge_slice_prune_table: box [[None;
                Cube3x3x3::EDGE_SLICE_INDEX_COUNT];
                Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT],
            edge_orientation_prune_table: box [[None; Cube3x3x3::EDGE_SLICE_INDEX_COUNT];
                Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT],
            combined_orientation_prune_table: box [[None; Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT];
                Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT],
            corner_edge_permutation_prune_table: box [[None;
                Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];
                Cube3x3x3::CORNER_PERMUTATION_INDEX_COUNT],
            phase_2_edge_permutation_prune_table: box [[None;
                Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT];
                Cube3x3x3::PHASE_2_EDGE_PERMUTATION_INDEX_COUNT],
            corner_orientation_prune_table: box [None; Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT],
            corner_permutation_prune_table: box [None; Cube3x3x3::CORNER_PERMUTATION_INDEX_COUNT],
        };
        tables.corner_orientation_edge_slice_prune_table[0][0] = Some(0);
        tables.edge_orientation_prune_table[0][0] = Some(0);
        tables.combined_orientation_prune_table[0][0] = Some(0);
        tables.corner_edge_permutation_prune_table[0][0] = Some(0);
        tables.phase_2_edge_permutation_prune_table[0][0] = Some(0);
        tables.corner_orientation_prune_table[0] = Some(0);
        tables.corner_permutation_prune_table[0] = Some(0);
        tables
    }

    fn corner_orientation_move_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.corner_orientation_move_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn corner_permutation_move_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.corner_permutation_move_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn edge_orientation_move_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.edge_orientation_move_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn equatorial_edge_slice_move_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.equatorial_edge_slice_move_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn phase_2_edge_permutation_move_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.phase_2_edge_permutation_move_table.as_ref() {
            for (j_idx, j) in i.iter().enumerate() {
                if Self::valid_phase_2_move(Move::try_from(j_idx as u8).unwrap()) {
                    total += 1;
                    if j.is_some() {
                        filled += 1;
                    }
                }
            }
        }
        Progress { filled, total }
    }

    fn phase_2_equatorial_edge_permutation_move_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.phase_2_equatorial_edge_permutation_move_table.as_ref() {
            for (j_idx, j) in i.iter().enumerate() {
                if Self::valid_phase_2_move(Move::try_from(j_idx as u8).unwrap()) {
                    total += 1;
                    if j.is_some() {
                        filled += 1;
                    }
                }
            }
        }
        Progress { filled, total }
    }

    fn corner_orientation_edge_slice_prune_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.corner_orientation_edge_slice_prune_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn edge_orientation_prune_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.edge_orientation_prune_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn combined_orientation_prune_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.combined_orientation_prune_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn corner_edge_permutation_prune_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.corner_edge_permutation_prune_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn phase_2_edge_permutation_prune_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.phase_2_edge_permutation_prune_table.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    fn corner_orientation_prune_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.corner_orientation_prune_table.as_ref() {
            total += 1;
            if i.is_some() {
                filled += 1;
            }
        }
        Progress { filled, total }
    }

    fn corner_permutation_prune_table_progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.corner_permutation_prune_table.as_ref() {
            total += 1;
            if i.is_some() {
                filled += 1;
            }
        }
        Progress { filled, total }
    }

    fn phase_1_move(&mut self, cubes: Vec<Cube3x3x3>) -> Vec<Cube3x3x3> {
        let mut next_cubes = Vec::new();

        for cube in cubes {
            for move_idx in 0..Move::count_3x3x3() {
                let mut cube = cube.clone();
                let mv = Move::try_from(move_idx as u8).unwrap();

                // Get old indicies so that we know where we came from
                let old_corner_orientation = cube.corner_orientation_index() as usize;
                let old_corner_permutation = cube.corner_permutation_index() as usize;
                let old_edge_orientation = cube.edge_orientation_index() as usize;
                let old_equatorial_slice = cube.equatorial_edge_slice_index() as usize;

                // Get current move counts for this cube
                let corner_orient_edge_slice_move_count = self
                    .corner_orientation_edge_slice_prune_table[old_corner_orientation]
                    [old_equatorial_slice]
                    .unwrap()
                    + 1;
                let edge_orient_move_count = self.edge_orientation_prune_table
                    [old_edge_orientation][old_equatorial_slice]
                    .unwrap()
                    + 1;
                let combined_orient_move_count = self.combined_orientation_prune_table
                    [old_corner_orientation][old_edge_orientation]
                    .unwrap()
                    + 1;
                let corner_orient_move_count =
                    self.corner_orientation_prune_table[old_corner_orientation].unwrap() + 1;
                let corner_permute_move_count =
                    self.corner_permutation_prune_table[old_corner_permutation].unwrap() + 1;

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_corner_orientation = cube.corner_orientation_index();
                let new_corner_permutation = cube.corner_permutation_index();
                let new_edge_orientation = cube.edge_orientation_index();
                let new_equatorial_slice = cube.equatorial_edge_slice_index();

                let mut has_new_info = false;

                // Update corner orientation move table
                if self.corner_orientation_move_table[old_corner_orientation][move_idx].is_none() {
                    self.corner_orientation_move_table[old_corner_orientation][move_idx] =
                        Some(new_corner_orientation);
                    has_new_info = true;
                } else {
                    assert_eq!(
                        self.corner_orientation_move_table[old_corner_orientation][move_idx],
                        Some(new_corner_orientation)
                    );
                }

                // Update corner permutation move table
                if self.corner_permutation_move_table[old_corner_permutation][move_idx].is_none() {
                    self.corner_permutation_move_table[old_corner_permutation][move_idx] =
                        Some(new_corner_permutation);
                    has_new_info = true;
                } else {
                    assert_eq!(
                        self.corner_permutation_move_table[old_corner_permutation][move_idx],
                        Some(new_corner_permutation)
                    );
                }

                // Update edge orientation move table
                if self.edge_orientation_move_table[old_edge_orientation][move_idx].is_none() {
                    self.edge_orientation_move_table[old_edge_orientation][move_idx] =
                        Some(new_edge_orientation);
                    has_new_info = true;
                } else {
                    assert_eq!(
                        self.edge_orientation_move_table[old_edge_orientation][move_idx],
                        Some(new_edge_orientation)
                    );
                }

                // Update equatorial edge slice move table
                if self.equatorial_edge_slice_move_table[old_equatorial_slice][move_idx].is_none() {
                    self.equatorial_edge_slice_move_table[old_equatorial_slice][move_idx] =
                        Some(new_equatorial_slice);
                    has_new_info = true;
                } else {
                    assert_eq!(
                        self.equatorial_edge_slice_move_table[old_equatorial_slice][move_idx],
                        Some(new_equatorial_slice)
                    );
                }

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                if self.corner_orientation_edge_slice_prune_table[new_corner_orientation as usize]
                    [new_equatorial_slice as usize]
                    .is_none()
                    || Some(corner_orient_edge_slice_move_count)
                        < self.corner_orientation_edge_slice_prune_table
                            [new_corner_orientation as usize]
                            [new_equatorial_slice as usize]
                {
                    self.corner_orientation_edge_slice_prune_table
                        [new_corner_orientation as usize][new_equatorial_slice as usize] =
                        Some(corner_orient_edge_slice_move_count);
                    has_new_info = true;
                }
                if self.edge_orientation_prune_table[new_edge_orientation as usize]
                    [new_equatorial_slice as usize]
                    .is_none()
                    || Some(edge_orient_move_count)
                        < self.edge_orientation_prune_table[new_edge_orientation as usize]
                            [new_equatorial_slice as usize]
                {
                    self.edge_orientation_prune_table[new_edge_orientation as usize]
                        [new_equatorial_slice as usize] = Some(edge_orient_move_count);
                    has_new_info = true;
                }
                if self.combined_orientation_prune_table[new_corner_orientation as usize]
                    [new_edge_orientation as usize]
                    .is_none()
                    || Some(combined_orient_move_count)
                        < self.combined_orientation_prune_table[new_corner_orientation as usize]
                            [new_edge_orientation as usize]
                {
                    self.combined_orientation_prune_table[new_corner_orientation as usize]
                        [new_edge_orientation as usize] = Some(combined_orient_move_count);
                    has_new_info = true;
                }
                if self.corner_orientation_prune_table[new_corner_orientation as usize].is_none()
                    || Some(corner_orient_move_count)
                        < self.corner_orientation_prune_table[new_corner_orientation as usize]
                {
                    self.corner_orientation_prune_table[new_corner_orientation as usize] =
                        Some(corner_orient_move_count);
                    has_new_info = true;
                }
                if self.corner_permutation_prune_table[new_corner_permutation as usize].is_none()
                    || Some(corner_permute_move_count)
                        < self.corner_permutation_prune_table[new_corner_permutation as usize]
                {
                    self.corner_permutation_prune_table[new_corner_permutation as usize] =
                        Some(corner_permute_move_count);
                    has_new_info = true;
                }

                // If there was new information discovered with this state, add it to the queue for processing
                if has_new_info {
                    next_cubes.push(cube);
                }
            }
        }

        next_cubes
    }

    fn valid_phase_2_move(mv: Move) -> bool {
        match mv {
            Move::F | Move::Fp | Move::R | Move::Rp | Move::B | Move::Bp | Move::L | Move::Lp => {
                false
            }
            _ => true,
        }
    }

    fn phase_2_move(&mut self, cubes: Vec<Cube3x3x3>) -> Vec<Cube3x3x3> {
        let mut next_cubes = Vec::new();

        for cube in cubes {
            for move_idx in 0..Move::count_3x3x3() {
                let mut cube = cube.clone();
                let mv = Move::try_from(move_idx as u8).unwrap();

                // Phase 2 contains the moves (U, D, F2, R2, B2, L2)
                if !Self::valid_phase_2_move(mv) {
                    continue;
                }

                // Get old indicies so that we know where we came from
                let old_corner_permutation = cube.corner_permutation_index();
                let old_edge_permutation = cube.phase_2_edge_permutation_index();
                let old_equatorial_edge_permutation =
                    cube.phase_2_equatorial_edge_permutation_index();

                // Get current move counts for this cube
                let corner_permute_move_count = self.corner_edge_permutation_prune_table
                    [old_corner_permutation as usize]
                    [old_equatorial_edge_permutation as usize]
                    .unwrap()
                    + 1;
                let edge_permute_move_count = self.phase_2_edge_permutation_prune_table
                    [old_edge_permutation as usize][old_equatorial_edge_permutation as usize]
                    .unwrap()
                    + 1;

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_corner_permutation = cube.corner_permutation_index();
                let new_edge_permutation = cube.phase_2_edge_permutation_index();
                let new_equatorial_edge_permutation =
                    cube.phase_2_equatorial_edge_permutation_index();

                let mut has_new_info = false;

                // Update edge permutation move table
                if self.phase_2_edge_permutation_move_table[old_edge_permutation as usize][move_idx]
                    .is_none()
                {
                    self.phase_2_edge_permutation_move_table[old_edge_permutation as usize]
                        [move_idx] = Some(new_edge_permutation);
                    has_new_info = true;
                } else {
                    assert_eq!(
                        self.phase_2_edge_permutation_move_table[old_edge_permutation as usize]
                            [move_idx],
                        Some(new_edge_permutation)
                    );
                }

                // Update equatorial edge slice move table
                if self.phase_2_equatorial_edge_permutation_move_table
                    [old_equatorial_edge_permutation as usize][move_idx]
                    .is_none()
                {
                    self.phase_2_equatorial_edge_permutation_move_table
                        [old_equatorial_edge_permutation as usize][move_idx] =
                        Some(new_equatorial_edge_permutation);
                    has_new_info = true;
                } else {
                    assert_eq!(
                        self.phase_2_equatorial_edge_permutation_move_table
                            [old_equatorial_edge_permutation as usize][move_idx],
                        Some(new_equatorial_edge_permutation)
                    );
                }

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                if self.corner_edge_permutation_prune_table[new_corner_permutation as usize]
                    [new_equatorial_edge_permutation as usize]
                    .is_none()
                    || Some(corner_permute_move_count)
                        < self.corner_edge_permutation_prune_table[new_corner_permutation as usize]
                            [new_equatorial_edge_permutation as usize]
                {
                    self.corner_edge_permutation_prune_table[new_corner_permutation as usize]
                        [new_equatorial_edge_permutation as usize] =
                        Some(corner_permute_move_count);
                    has_new_info = true;
                }
                if self.phase_2_edge_permutation_prune_table[new_edge_permutation as usize]
                    [new_equatorial_edge_permutation as usize]
                    .is_none()
                    || Some(edge_permute_move_count)
                        < self.phase_2_edge_permutation_prune_table[new_edge_permutation as usize]
                            [new_equatorial_edge_permutation as usize]
                {
                    self.phase_2_edge_permutation_prune_table[new_edge_permutation as usize]
                        [new_equatorial_edge_permutation as usize] = Some(edge_permute_move_count);
                    has_new_info = true;
                }

                // If there was new information discovered with this state, add it to the queue for processing
                if has_new_info {
                    next_cubes.push(cube);
                }
            }
        }

        next_cubes
    }

    pub fn generate(&mut self) {
        // Generate all tables for phase 1 of the solve
        println!("Generating phase 1 tables...");
        let mut active_cubes = Vec::new();
        active_cubes.push(Cube3x3x3::new());
        let mut i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("    Phase 1 move {}", i);
            active_cubes = self.phase_1_move(active_cubes);
            println!("        {} active cube states", active_cubes.len());
            println!(
                "        {} corner orientation move table",
                self.corner_orientation_move_table_progress()
            );
            println!(
                "        {} corner permutation move table",
                self.corner_permutation_move_table_progress()
            );
            println!(
                "        {} edge orientation move table",
                self.edge_orientation_move_table_progress()
            );
            println!(
                "        {} equatorial edge slice move table",
                self.equatorial_edge_slice_move_table_progress()
            );
            println!(
                "        {} corner orientation / edge slice prune table",
                self.corner_orientation_edge_slice_prune_table_progress()
            );
            println!(
                "        {} edge orientation prune table",
                self.edge_orientation_prune_table_progress()
            );
            println!(
                "        {} combined orientation prune table",
                self.combined_orientation_prune_table_progress()
            );
            println!(
                "        {} corner orientation prune table",
                self.corner_orientation_prune_table_progress()
            );
            println!(
                "        {} corner permutation prune table",
                self.corner_permutation_prune_table_progress()
            );
        }

        // Generate all tables for phase 2 of the solve
        println!("Generating phase 2 tables...");
        active_cubes.push(Cube3x3x3::new());
        i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("    Phase 2 move {}", i);
            active_cubes = self.phase_2_move(active_cubes);
            println!("        {} active cube states", active_cubes.len());
            println!(
                "        {} edge permutation move table",
                self.phase_2_edge_permutation_move_table_progress()
            );
            println!(
                "        {} equatorial edge slice move table",
                self.phase_2_equatorial_edge_permutation_move_table_progress()
            );
            println!(
                "        {} corner / edge permutation prune table",
                self.corner_edge_permutation_prune_table_progress()
            );
            println!(
                "        {} edge permutation prune table",
                self.phase_2_edge_permutation_prune_table_progress()
            );
        }

        // Ensure that all tables have been filled in completely
        assert!(self.corner_orientation_move_table_progress().complete());
        assert!(self.corner_permutation_move_table_progress().complete());
        assert!(self.edge_orientation_move_table_progress().complete());
        assert!(self.equatorial_edge_slice_move_table_progress().complete());
        assert!(self
            .phase_2_edge_permutation_move_table_progress()
            .complete());
        assert!(self
            .phase_2_equatorial_edge_permutation_move_table_progress()
            .complete());
        assert!(self
            .corner_orientation_edge_slice_prune_table_progress()
            .complete());
        assert!(self.edge_orientation_prune_table_progress().complete());
        assert!(self.combined_orientation_prune_table_progress().complete());
        assert!(self
            .corner_edge_permutation_prune_table_progress()
            .complete());
        assert!(self
            .phase_2_edge_permutation_prune_table_progress()
            .complete());
        assert!(self.corner_orientation_prune_table_progress().complete());
        assert!(self.corner_permutation_prune_table_progress().complete());

        // Output tables
        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/corner_orientation_move_table.bin",
            ))
            .unwrap(),
        );
        for i in self.corner_orientation_move_table.as_ref() {
            for j in i {
                out.write(&(j.unwrap() as u16).to_le_bytes()).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/corner_permutation_move_table.bin",
            ))
            .unwrap(),
        );
        for i in self.corner_permutation_move_table.as_ref() {
            for j in i {
                out.write(&(j.unwrap() as u16).to_le_bytes()).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_edge_orientation_move_table.bin",
            ))
            .unwrap(),
        );
        for i in self.edge_orientation_move_table.as_ref() {
            for j in i {
                out.write(&(j.unwrap() as u16).to_le_bytes()).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_equatorial_edge_slice_move_table.bin",
            ))
            .unwrap(),
        );
        for i in self.equatorial_edge_slice_move_table.as_ref() {
            for j in i {
                out.write(&(j.unwrap() as u16).to_le_bytes()).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_phase_2_edge_permutation_move_table.bin",
            ))
            .unwrap(),
        );
        for i in self.phase_2_edge_permutation_move_table.as_ref() {
            for j in i {
                let j = j.or(Some(0)).unwrap();
                out.write(&(j as u16).to_le_bytes()).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_phase_2_equatorial_edge_permutation_move_table.bin",
            ))
            .unwrap(),
        );
        for i in self.phase_2_equatorial_edge_permutation_move_table.as_ref() {
            for j in i {
                let j = j.or(Some(0)).unwrap();
                out.write(&(j as u16).to_le_bytes()).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_corner_orientation_edge_slice_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.corner_orientation_edge_slice_prune_table.as_ref() {
            for j in i {
                out.write(&[j.unwrap() as u8]).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_edge_orientation_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.edge_orientation_prune_table.as_ref() {
            for j in i {
                out.write(&[j.unwrap() as u8]).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_combined_orientation_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.combined_orientation_prune_table.as_ref() {
            for j in i {
                out.write(&[j.unwrap() as u8]).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_corner_edge_permutation_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.corner_edge_permutation_prune_table.as_ref() {
            for j in i {
                out.write(&[j.unwrap() as u8]).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_phase_2_edge_permutation_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.phase_2_edge_permutation_prune_table.as_ref() {
            for j in i {
                out.write(&[j.unwrap() as u8]).unwrap();
            }
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/3x3x3_phase_1_corner_permutation_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.corner_edge_permutation_prune_table.as_ref() {
            let mut min = i[0].unwrap();
            for j in i {
                min = std::cmp::min(min, j.unwrap());
            }
            out.write(&[min as u8]).unwrap();
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/corner_orientation_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.corner_orientation_prune_table.as_ref() {
            out.write(&[i.unwrap() as u8]).unwrap();
        }

        let mut out = BufWriter::new(
            File::create(Path::new(
                "../../lib/src/tables/corner_permutation_prune_table.bin",
            ))
            .unwrap(),
        );
        for i in self.corner_permutation_prune_table.as_ref() {
            out.write(&[i.unwrap() as u8]).unwrap();
        }
    }
}

fn main() {
    let mut tables = TableGenerator::new();
    tables.generate();
}
