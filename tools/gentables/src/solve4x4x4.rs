use crate::common::{MoveTable, PruneTable1D, PruneTable2D};
use std::convert::TryFrom;
use std::vec::Vec;
use tpscube_core::{Cube, Cube4x4x4, InitialCubeState, Move};

#[derive(Default)]
pub struct TableGenerator {
    corner_orientation_move_table:
        MoveTable<{ Cube4x4x4::CORNER_ORIENTATION_INDEX_COUNT }, { Move::count_4x4x4() }>,
    corner_permutation_move_table:
        MoveTable<{ Cube4x4x4::CORNER_PERMUTATION_INDEX_COUNT }, { Move::count_4x4x4() }>,
    phase_1_red_centers_move_table:
        MoveTable<{ Cube4x4x4::PHASE_1_RED_ORANGE_CENTERS_INDEX_COUNT }, { Move::count_4x4x4() }>,
    phase_1_orange_centers_move_table:
        MoveTable<{ Cube4x4x4::PHASE_1_RED_ORANGE_CENTERS_INDEX_COUNT }, { Move::count_4x4x4() }>,
    phase_2_red_orange_centers_move_table:
        MoveTable<{ Cube4x4x4::PHASE_2_RED_ORANGE_CENTERS_INDEX_COUNT }, { Move::count_4x4x4() }>,
    phase_2_green_blue_centers_move_table:
        MoveTable<{ Cube4x4x4::PHASE_2_GREEN_BLUE_CENTERS_INDEX_COUNT }, { Move::count_4x4x4() }>,
    phase_3_green_blue_centers_move_table:
        MoveTable<{ Cube4x4x4::PHASE_3_GREEN_BLUE_CENTERS_INDEX_COUNT }, { Move::count_4x4x4() }>,
    phase_4_centers_move_table:
        MoveTable<{ Cube4x4x4::PHASE_4_CENTERS_INDEX_COUNT }, { Move::count_4x4x4() }>,
    phase_4_edge_pair_move_table:
        MoveTable<{ Cube4x4x4::PHASE_4_EDGE_PAIR_INDEX_COUNT }, { Move::count_4x4x4() }>,
    corner_orientation_prune_table: PruneTable1D<{ Cube4x4x4::CORNER_ORIENTATION_INDEX_COUNT }>,
    corner_permutation_prune_table: PruneTable1D<{ Cube4x4x4::CORNER_PERMUTATION_INDEX_COUNT }>,
    phase_1_red_centers_prune_table:
        PruneTable1D<{ Cube4x4x4::PHASE_1_RED_ORANGE_CENTERS_INDEX_COUNT }>,
    phase_1_orange_centers_prune_table:
        PruneTable1D<{ Cube4x4x4::PHASE_1_RED_ORANGE_CENTERS_INDEX_COUNT }>,
    phase_2_centers_prune_table: PruneTable2D<
        { Cube4x4x4::PHASE_2_RED_ORANGE_CENTERS_INDEX_COUNT },
        { Cube4x4x4::PHASE_2_GREEN_BLUE_CENTERS_INDEX_COUNT },
    >,
    phase_3_centers_prune_table: PruneTable2D<
        { Cube4x4x4::PHASE_3_RED_ORANGE_CENTERS_INDEX_COUNT },
        { Cube4x4x4::PHASE_3_GREEN_BLUE_CENTERS_INDEX_COUNT },
    >,
    phase_4_centers_prune_table: PruneTable1D<{ Cube4x4x4::PHASE_4_CENTERS_INDEX_COUNT }>,
    phase_4_edge_pair_prune_table: PruneTable1D<{ Cube4x4x4::PHASE_4_EDGE_PAIR_INDEX_COUNT }>,
}

impl TableGenerator {
    pub fn new() -> Self {
        let mut tables = Self::default();
        tables.corner_orientation_prune_table.set(0, 0);
        tables.corner_permutation_prune_table.set(0, 0);
        tables.phase_1_red_centers_prune_table.set(0, 0);
        tables.phase_1_orange_centers_prune_table.set(0, 0);
        tables.phase_2_centers_prune_table.set(0, 0, 0);
        tables.phase_3_centers_prune_table.set(0, 0, 0);
        tables.phase_4_centers_prune_table.set(0, 0);
        tables.phase_4_edge_pair_prune_table.set(0, 0);
        tables
    }

    fn phase_1_move(&mut self, cubes: Vec<Cube4x4x4>) -> Vec<Cube4x4x4> {
        let mut next_cubes = Vec::new();

        for cube in cubes {
            for move_idx in 0..Move::count_4x4x4() {
                let mut cube = cube.clone();
                let mv = Move::try_from(move_idx as u8).unwrap();

                // Get old indicies so that we know where we came from
                let old_corner_orientation = cube.corner_orientation_index();
                let old_corner_permutation = cube.corner_permutation_index();
                let old_red_centers = cube.phase_1_red_centers_index();
                let old_orange_centers = cube.phase_1_orange_centers_index();

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_corner_orientation = cube.corner_orientation_index();
                let new_corner_permutation = cube.corner_permutation_index();
                let new_red_centers = cube.phase_1_red_centers_index();
                let new_orange_centers = cube.phase_1_orange_centers_index();

                // Update move tables
                let mut has_new_info = self.corner_orientation_move_table.update(
                    old_corner_orientation,
                    mv,
                    new_corner_orientation,
                );
                has_new_info |= self.corner_permutation_move_table.update(
                    old_corner_permutation,
                    mv,
                    new_corner_permutation,
                );
                has_new_info |= self.phase_1_red_centers_move_table.update(
                    old_red_centers,
                    mv,
                    new_red_centers,
                );
                has_new_info |= self.phase_1_orange_centers_move_table.update(
                    old_orange_centers,
                    mv,
                    new_orange_centers,
                );

                // Set prune table to zero if new state has red and orange centers on either the red
                // or orange face. This will allow easy detection of this case by looking for the zero
                // in both prune tables.
                if cube.red_centers_on_red_or_orange_face() {
                    has_new_info |= self
                        .phase_1_red_centers_prune_table
                        .update_as_solution(new_red_centers);
                }
                if cube.orange_centers_on_red_or_orange_face() {
                    has_new_info |= self
                        .phase_1_orange_centers_prune_table
                        .update_as_solution(new_orange_centers);
                }

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                has_new_info |= self
                    .corner_orientation_prune_table
                    .update(old_corner_orientation, new_corner_orientation);
                has_new_info |= self
                    .corner_permutation_prune_table
                    .update(old_corner_permutation, new_corner_permutation);
                has_new_info |= self
                    .phase_1_red_centers_prune_table
                    .update(old_red_centers, new_red_centers);
                has_new_info |= self
                    .phase_1_orange_centers_prune_table
                    .update(old_orange_centers, new_orange_centers);

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
            Move::Fw
            | Move::Fwp
            | Move::Uw
            | Move::Uwp
            | Move::Bw
            | Move::Bwp
            | Move::Dw
            | Move::Dwp => false,
            _ => true,
        }
    }

    fn phase_2_move(&mut self, cubes: Vec<Cube4x4x4>) -> Vec<Cube4x4x4> {
        let mut next_cubes = Vec::new();

        for cube in cubes {
            for move_idx in 0..Move::count_4x4x4() {
                let mut cube = cube.clone();
                let mv = Move::try_from(move_idx as u8).unwrap();

                // Phase 2 contains the moves (U, D, F, R, B, L, Uw2, Dw2, Fw2, Rw, Bw2, Lw)
                if !Self::valid_phase_2_move(mv) {
                    continue;
                }

                // Get old indicies so that we know where we came from
                let old_red_orange_centers = cube.phase_2_and_3_red_orange_centers_index();
                let old_green_blue_centers = cube.phase_2_green_blue_centers_index();

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_red_orange_centers = cube.phase_2_and_3_red_orange_centers_index();
                let new_green_blue_centers = cube.phase_2_green_blue_centers_index();

                // Update move tables
                let mut has_new_info = self.phase_2_red_orange_centers_move_table.update(
                    old_red_orange_centers,
                    mv,
                    new_red_orange_centers,
                );
                has_new_info |= self.phase_2_green_blue_centers_move_table.update(
                    old_green_blue_centers,
                    mv,
                    new_green_blue_centers,
                );

                // Set prune table to zero if new state has centers in a state that is
                // valid for the rest of the solve. This will allow easy detection of
                // this case by looking for the zero in the prune table.
                if cube.phase_2_centers_solved() {
                    has_new_info |= self
                        .phase_2_centers_prune_table
                        .update_as_solution(new_red_orange_centers, new_green_blue_centers);
                }

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                has_new_info |= self.phase_2_centers_prune_table.update(
                    old_red_orange_centers,
                    old_green_blue_centers,
                    new_red_orange_centers,
                    new_green_blue_centers,
                );

                // If there was new information discovered with this state, add it to the queue for processing
                if has_new_info {
                    next_cubes.push(cube);
                }
            }
        }

        next_cubes
    }

    fn valid_phase_3_move(mv: Move) -> bool {
        match mv {
            Move::R
            | Move::Rp
            | Move::L
            | Move::Lp
            | Move::Lw
            | Move::Lwp
            | Move::Rw
            | Move::Rwp
            | Move::Fw
            | Move::Fwp
            | Move::Uw
            | Move::Uwp
            | Move::Bw
            | Move::Bwp
            | Move::Dw
            | Move::Dwp => false,
            _ => true,
        }
    }

    fn phase_3_move(&mut self, cubes: Vec<Cube4x4x4>) -> Vec<Cube4x4x4> {
        let mut next_cubes = Vec::new();

        for cube in cubes {
            for move_idx in 0..Move::count_4x4x4() {
                let mut cube = cube.clone();
                let mv = Move::try_from(move_idx as u8).unwrap();

                // Phase 3 contains the moves (U, D, F, R2, B, L2, Uw2, Dw2, Fw2, Rw2, Bw2, Lw2)
                if !Self::valid_phase_3_move(mv) {
                    continue;
                }

                // Get old indicies so that we know where we came from
                let old_red_orange_centers = cube.phase_2_and_3_red_orange_centers_index();
                let old_green_blue_centers = cube.phase_3_green_blue_centers_index();

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_red_orange_centers = cube.phase_2_and_3_red_orange_centers_index();
                let new_green_blue_centers = cube.phase_3_green_blue_centers_index();

                // Update move tables
                let mut has_new_info = self.phase_3_green_blue_centers_move_table.update(
                    old_green_blue_centers,
                    mv,
                    new_green_blue_centers,
                );

                // Set prune table to zero if new state has centers in a state that is
                // valid for the rest of the solve. This will allow easy detection of
                // this case by looking for the zero in the prune table.
                if cube.phase_3_centers_solved() {
                    has_new_info |= self
                        .phase_3_centers_prune_table
                        .update_as_solution(new_red_orange_centers, new_green_blue_centers);
                }

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                has_new_info |= self.phase_3_centers_prune_table.update(
                    old_red_orange_centers,
                    old_green_blue_centers,
                    new_red_orange_centers,
                    new_green_blue_centers,
                );

                // If there was new information discovered with this state, add it to the queue for processing
                if has_new_info {
                    next_cubes.push(cube);
                }
            }
        }

        next_cubes
    }

    fn valid_phase_4_move(mv: Move) -> bool {
        match mv {
            Move::R
            | Move::Rp
            | Move::L
            | Move::Lp
            | Move::F
            | Move::Fp
            | Move::B
            | Move::Bp
            | Move::Lw
            | Move::Lwp
            | Move::Rw
            | Move::Rwp
            | Move::Fw
            | Move::Fwp
            | Move::Uw
            | Move::Uwp
            | Move::Uw2
            | Move::Bw
            | Move::Bwp
            | Move::Dw
            | Move::Dwp
            | Move::Dw2 => false,
            _ => true,
        }
    }

    fn phase_4_move(&mut self, cubes: Vec<Cube4x4x4>) -> Vec<Cube4x4x4> {
        let mut next_cubes = Vec::new();

        for cube in cubes {
            for move_idx in 0..Move::count_4x4x4() {
                let mut cube = cube.clone();
                let mv = Move::try_from(move_idx as u8).unwrap();

                // Phase 4 contains the moves (U, D, F2, R2, B2, L2, Fw2, Rw2, Bw2, Lw2)
                if !Self::valid_phase_4_move(mv) {
                    continue;
                }

                // Get old indicies so that we know where we came from
                let old_centers = cube.phase_4_centers_index();
                let old_edge_pair = cube.phase_4_edge_pair_index();

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_centers = cube.phase_4_centers_index();
                let new_edge_pair = cube.phase_4_edge_pair_index();

                // Update move tables
                let mut has_new_info =
                    self.phase_4_centers_move_table
                        .update(old_centers, mv, new_centers);
                has_new_info |=
                    self.phase_4_edge_pair_move_table
                        .update(old_edge_pair, mv, new_edge_pair);

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                has_new_info |= self
                    .phase_4_centers_prune_table
                    .update(old_centers, new_centers);
                has_new_info |= self
                    .phase_4_edge_pair_prune_table
                    .update(old_edge_pair, new_edge_pair);

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
        let mut active_cubes = Vec::new();
        active_cubes.push(Cube4x4x4::new());
        let mut i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("4x4x4 Phase 1 move {}", i);
            active_cubes = self.phase_1_move(active_cubes);
            println!("    {} active cube states", active_cubes.len());
            println!(
                "    {} corner orientation move table",
                self.corner_orientation_move_table.progress()
            );
            println!(
                "    {} corner permutation move table",
                self.corner_permutation_move_table.progress()
            );
            println!(
                "    {} red centers move table",
                self.phase_1_red_centers_move_table.progress()
            );
            println!(
                "    {} orange centers move table",
                self.phase_1_orange_centers_move_table.progress()
            );
            println!(
                "    {} corner orientation prune table",
                self.corner_orientation_prune_table.progress()
            );
            println!(
                "    {} corner permutation prune table",
                self.corner_permutation_prune_table.progress()
            );
            println!(
                "    {} red centers prune table",
                self.phase_1_red_centers_prune_table.progress()
            );
            println!(
                "    {} orange centers prune table",
                self.phase_1_orange_centers_prune_table.progress()
            );
        }

        // Generate all tables for phase 2 of the solve
        let mut active_cubes = Vec::new();
        active_cubes.push(Cube4x4x4::new());
        let mut i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("4x4x4 Phase 2 move {}", i);
            active_cubes = self.phase_2_move(active_cubes);
            println!("    {} active cube states", active_cubes.len());
            println!(
                "    {} red orange centers move table",
                self.phase_2_red_orange_centers_move_table
                    .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            );
            println!(
                "    {} green blue centers move table",
                self.phase_2_green_blue_centers_move_table
                    .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            );
            println!(
                "    {} centers prune table",
                self.phase_2_centers_prune_table.progress()
            );
        }

        // Generate all tables for phase 3 of the solve
        let mut active_cubes = Vec::new();
        active_cubes.push(Cube4x4x4::new());
        let mut i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("4x4x4 Phase 3 move {}", i);
            active_cubes = self.phase_3_move(active_cubes);
            println!("    {} active cube states", active_cubes.len());
            println!(
                "    {} green blue centers move table",
                self.phase_3_green_blue_centers_move_table
                    .progress_filtered(|mv| Self::valid_phase_3_move(mv))
            );
            println!(
                "    {} centers prune table",
                self.phase_3_centers_prune_table.progress()
            );
        }

        // Generate all tables for phase 4 of the solve
        let mut active_cubes = Vec::new();
        active_cubes.push(Cube4x4x4::new());
        let mut i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("4x4x4 Phase 4 move {}", i);
            active_cubes = self.phase_4_move(active_cubes);
            println!("    {} active cube states", active_cubes.len());
            println!(
                "    {} centers move table",
                self.phase_4_centers_move_table
                    .progress_filtered(|mv| Self::valid_phase_4_move(mv))
            );
            println!(
                "    {} edge pair move table",
                self.phase_4_edge_pair_move_table
                    .progress_filtered(|mv| Self::valid_phase_4_move(mv))
            );
            println!(
                "    {} centers prune table",
                self.phase_4_centers_prune_table.progress()
            );
            println!(
                "    {} edge pair prune table",
                self.phase_4_edge_pair_prune_table.progress()
            );
        }

        // Ensure that all tables have been filled in completely
        assert!(self.corner_orientation_move_table.progress().complete());
        assert!(self.corner_permutation_move_table.progress().complete());
        assert!(self.phase_1_red_centers_move_table.progress().complete());
        assert!(self.phase_1_orange_centers_move_table.progress().complete());
        assert!(self
            .phase_2_red_orange_centers_move_table
            .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            .complete());
        assert!(self
            .phase_2_green_blue_centers_move_table
            .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            .complete());
        assert!(self
            .phase_3_green_blue_centers_move_table
            .progress_filtered(|mv| Self::valid_phase_3_move(mv))
            .complete());
        assert!(self
            .phase_4_centers_move_table
            .progress_filtered(|mv| Self::valid_phase_4_move(mv))
            .complete());
        // Phase 4 edge pairing only fills half because of solved parity
        // assert!(self
        //    .phase_4_edge_pair_move_table
        //    .progress_filtered(|mv| Self::valid_phase_4_move(mv))
        //    .complete());
        assert!(self.corner_orientation_prune_table.progress().complete());
        assert!(self.corner_permutation_prune_table.progress().complete());
        assert!(self.phase_1_red_centers_prune_table.progress().complete());
        assert!(self
            .phase_1_orange_centers_prune_table
            .progress()
            .complete());
        assert!(self.phase_2_centers_prune_table.progress().complete());
        // Phase 3 centers not verified, as not every state is actually reachable (the
        // index has more states than actually necessary).
        // assert!(self.phase_3_centers_prune_table.progress().complete());
        assert!(self.phase_4_centers_prune_table.progress().complete());
        // Phase 4 edge pairing only fills half because of solved parity
        // assert!(self.phase_4_edge_pair_prune_table.progress().complete());

        // Output tables
        self.corner_orientation_move_table
            .write("../../lib/src/tables/corner_orientation_move_table.bin");
        self.corner_permutation_move_table
            .write("../../lib/src/tables/corner_permutation_move_table.bin");
        self.phase_1_red_centers_move_table
            .write("../../lib/src/tables/4x4x4_phase_1_red_centers_move_table.bin");
        self.phase_1_orange_centers_move_table
            .write("../../lib/src/tables/4x4x4_phase_1_orange_centers_move_table.bin");
        self.phase_2_red_orange_centers_move_table
            .write("../../lib/src/tables/4x4x4_phase_2_red_orange_centers_move_table.bin");
        self.phase_2_green_blue_centers_move_table
            .write("../../lib/src/tables/4x4x4_phase_2_green_blue_centers_move_table.bin");
        self.phase_3_green_blue_centers_move_table
            .write("../../lib/src/tables/4x4x4_phase_3_green_blue_centers_move_table.bin");
        self.phase_4_centers_move_table
            .write("../../lib/src/tables/4x4x4_phase_4_centers_move_table.bin");
        self.phase_4_edge_pair_move_table
            .write("../../lib/src/tables/4x4x4_phase_4_edge_pair_move_table.bin");
        self.corner_orientation_prune_table
            .write("../../lib/src/tables/corner_orientation_prune_table.bin");
        self.corner_permutation_prune_table
            .write("../../lib/src/tables/corner_permutation_prune_table.bin");
        self.phase_1_red_centers_prune_table
            .write("../../lib/src/tables/4x4x4_phase_1_red_centers_prune_table.bin");
        self.phase_1_orange_centers_prune_table
            .write("../../lib/src/tables/4x4x4_phase_1_orange_centers_prune_table.bin");
        self.phase_2_centers_prune_table
            .write("../../lib/src/tables/4x4x4_phase_2_centers_prune_table.bin");
        self.phase_3_centers_prune_table
            .write("../../lib/src/tables/4x4x4_phase_3_centers_prune_table.bin");
        self.phase_4_centers_prune_table
            .write("../../lib/src/tables/4x4x4_phase_4_centers_prune_table.bin");
        self.phase_4_edge_pair_prune_table
            .write("../../lib/src/tables/4x4x4_phase_4_edge_pair_prune_table.bin");
    }
}
