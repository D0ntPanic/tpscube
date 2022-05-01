use crate::common::{MoveTable, PruneTable2D};
use std::convert::TryFrom;
use std::vec::Vec;
use tpscube_core::{Cube, Cube3x3x3, InitialCubeState, Move};

#[derive(Default)]
pub struct TableGenerator {
    edge_orientation_move_table:
        MoveTable<{ Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT }, { Move::count_3x3x3() }>,
    equatorial_edge_slice_move_table:
        MoveTable<{ Cube3x3x3::EDGE_SLICE_INDEX_COUNT }, { Move::count_3x3x3() }>,
    phase_2_edge_permutation_move_table:
        MoveTable<{ Cube3x3x3::PHASE_2_EDGE_PERMUTATION_INDEX_COUNT }, { Move::count_3x3x3() }>,
    phase_2_equatorial_edge_permutation_move_table: MoveTable<
        { Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT },
        { Move::count_3x3x3() },
    >,
    corner_orientation_edge_slice_prune_table: PruneTable2D<
        { Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT },
        { Cube3x3x3::EDGE_SLICE_INDEX_COUNT },
    >,
    edge_orientation_prune_table: PruneTable2D<
        { Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT },
        { Cube3x3x3::EDGE_SLICE_INDEX_COUNT },
    >,
    combined_orientation_prune_table: PruneTable2D<
        { Cube3x3x3::CORNER_ORIENTATION_INDEX_COUNT },
        { Cube3x3x3::EDGE_ORIENTATION_INDEX_COUNT },
    >,
    corner_edge_permutation_prune_table: PruneTable2D<
        { Cube3x3x3::CORNER_PERMUTATION_INDEX_COUNT },
        { Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT },
    >,
    phase_2_edge_permutation_prune_table: PruneTable2D<
        { Cube3x3x3::PHASE_2_EDGE_PERMUTATION_INDEX_COUNT },
        { Cube3x3x3::PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT },
    >,
}

impl TableGenerator {
    pub fn new() -> Self {
        let mut tables = Self::default();
        tables
            .corner_orientation_edge_slice_prune_table
            .set(0, 0, 0);
        tables.edge_orientation_prune_table.set(0, 0, 0);
        tables.combined_orientation_prune_table.set(0, 0, 0);
        tables.corner_edge_permutation_prune_table.set(0, 0, 0);
        tables.phase_2_edge_permutation_prune_table.set(0, 0, 0);
        tables
    }

    fn phase_1_move(&mut self, cubes: Vec<Cube3x3x3>) -> Vec<Cube3x3x3> {
        let mut next_cubes = Vec::new();

        for cube in cubes {
            for move_idx in 0..Move::count_3x3x3() {
                let mut cube = cube.clone();
                let mv = Move::try_from(move_idx as u8).unwrap();

                // Get old indicies so that we know where we came from
                let old_corner_orientation = cube.corner_orientation_index();
                let old_edge_orientation = cube.edge_orientation_index();
                let old_equatorial_slice = cube.equatorial_edge_slice_index();

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_corner_orientation = cube.corner_orientation_index();
                let new_edge_orientation = cube.edge_orientation_index();
                let new_equatorial_slice = cube.equatorial_edge_slice_index();

                // Update move tables
                let mut has_new_info = self.edge_orientation_move_table.update(
                    old_edge_orientation,
                    mv,
                    new_edge_orientation,
                );
                has_new_info |= self.equatorial_edge_slice_move_table.update(
                    old_equatorial_slice,
                    mv,
                    new_equatorial_slice,
                );

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                has_new_info |= self.corner_orientation_edge_slice_prune_table.update(
                    old_corner_orientation,
                    old_equatorial_slice,
                    new_corner_orientation,
                    new_equatorial_slice,
                );
                has_new_info |= self.edge_orientation_prune_table.update(
                    old_edge_orientation,
                    old_equatorial_slice,
                    new_edge_orientation,
                    new_equatorial_slice,
                );
                has_new_info |= self.combined_orientation_prune_table.update(
                    old_corner_orientation,
                    old_edge_orientation,
                    new_corner_orientation,
                    new_edge_orientation,
                );

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

                // Perform the move
                cube.do_move(mv);

                // Get new indicies for this state
                let new_corner_permutation = cube.corner_permutation_index();
                let new_edge_permutation = cube.phase_2_edge_permutation_index();
                let new_equatorial_edge_permutation =
                    cube.phase_2_equatorial_edge_permutation_index();

                // Update move tables
                let mut has_new_info = self.phase_2_edge_permutation_move_table.update(
                    old_edge_permutation,
                    mv,
                    new_edge_permutation,
                );
                has_new_info |= self.phase_2_equatorial_edge_permutation_move_table.update(
                    old_equatorial_edge_permutation,
                    mv,
                    new_equatorial_edge_permutation,
                );

                // Update prune tables to keep track of minimum number of moves to reach this state from solved
                has_new_info |= self.corner_edge_permutation_prune_table.update(
                    old_corner_permutation,
                    old_equatorial_edge_permutation,
                    new_corner_permutation,
                    new_equatorial_edge_permutation,
                );
                has_new_info |= self.phase_2_edge_permutation_prune_table.update(
                    old_edge_permutation,
                    old_equatorial_edge_permutation,
                    new_edge_permutation,
                    new_equatorial_edge_permutation,
                );

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
        active_cubes.push(Cube3x3x3::new());
        let mut i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("3x3x3 Phase 1 move {}", i);
            active_cubes = self.phase_1_move(active_cubes);
            println!("    {} active cube states", active_cubes.len());
            println!(
                "    {} edge orientation move table",
                self.edge_orientation_move_table.progress()
            );
            println!(
                "    {} equatorial edge slice move table",
                self.equatorial_edge_slice_move_table.progress()
            );
            println!(
                "    {} corner orientation / edge slice prune table",
                self.corner_orientation_edge_slice_prune_table.progress()
            );
            println!(
                "    {} edge orientation prune table",
                self.edge_orientation_prune_table.progress()
            );
            println!(
                "    {} combined orientation prune table",
                self.combined_orientation_prune_table.progress()
            );
        }

        // Generate all tables for phase 2 of the solve
        active_cubes.push(Cube3x3x3::new());
        i = 0;
        while active_cubes.len() > 0 {
            i += 1;
            println!("3x3x3 Phase 2 move {}", i);
            active_cubes = self.phase_2_move(active_cubes);
            println!("    {} active cube states", active_cubes.len());
            println!(
                "    {} edge permutation move table",
                self.phase_2_edge_permutation_move_table
                    .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            );
            println!(
                "    {} equatorial edge slice move table",
                self.phase_2_equatorial_edge_permutation_move_table
                    .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            );
            println!(
                "    {} corner / edge permutation prune table",
                self.corner_edge_permutation_prune_table.progress()
            );
            println!(
                "    {} edge permutation prune table",
                self.phase_2_edge_permutation_prune_table.progress()
            );
        }

        // Ensure that all tables have been filled in completely
        assert!(self.edge_orientation_move_table.progress().complete());
        assert!(self.equatorial_edge_slice_move_table.progress().complete());
        assert!(self
            .phase_2_edge_permutation_move_table
            .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            .complete());
        assert!(self
            .phase_2_equatorial_edge_permutation_move_table
            .progress_filtered(|mv| Self::valid_phase_2_move(mv))
            .complete());
        assert!(self
            .corner_orientation_edge_slice_prune_table
            .progress()
            .complete());
        assert!(self.edge_orientation_prune_table.progress().complete());
        assert!(self.combined_orientation_prune_table.progress().complete());
        assert!(self
            .corner_edge_permutation_prune_table
            .progress()
            .complete());
        assert!(self
            .phase_2_edge_permutation_prune_table
            .progress()
            .complete());

        // Output tables
        self.edge_orientation_move_table
            .write("../../lib/src/tables/3x3x3_edge_orientation_move_table.bin");
        self.equatorial_edge_slice_move_table
            .write("../../lib/src/tables/3x3x3_equatorial_edge_slice_move_table.bin");
        self.phase_2_edge_permutation_move_table
            .write("../../lib/src/tables/3x3x3_phase_2_edge_permutation_move_table.bin");
        self.phase_2_equatorial_edge_permutation_move_table
            .write("../../lib/src/tables/3x3x3_phase_2_equatorial_edge_permutation_move_table.bin");
        self.corner_orientation_edge_slice_prune_table
            .write("../../lib/src/tables/3x3x3_corner_orientation_edge_slice_prune_table.bin");
        self.edge_orientation_prune_table
            .write("../../lib/src/tables/3x3x3_edge_orientation_prune_table.bin");
        self.combined_orientation_prune_table
            .write("../../lib/src/tables/3x3x3_combined_orientation_prune_table.bin");
        self.corner_edge_permutation_prune_table
            .write("../../lib/src/tables/3x3x3_corner_edge_permutation_prune_table.bin");
        self.phase_2_edge_permutation_prune_table
            .write("../../lib/src/tables/3x3x3_phase_2_edge_permutation_prune_table.bin");
        self.corner_edge_permutation_prune_table
            .write_min("../../lib/src/tables/3x3x3_phase_1_corner_permutation_prune_table.bin");
    }
}
