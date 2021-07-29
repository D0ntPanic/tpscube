use super::{AlgorithmCounts, AlgorithmStats, AlgorithmType, Sort, SortColumn, SortOrder};
use egui::Ui;
use tpscube_core::{OLLAlgorithm, PLLAlgorithm};

const REQUIRED_COUNT: usize = 10;

pub(super) struct TPSReport<'a> {
    rows: Vec<AlgorithmRow>,
    sort: &'a mut Sort,
}

struct AlgorithmRow {
    algorithm: Algorithm,
    count: usize,
    moves: f32,
    recognition_time: f32,
    execution_time: f32,
    tps: f32,
    execution_tps: f32,
}

enum Algorithm {
    OLL(OLLAlgorithm),
    PLL(PLLAlgorithm),
}

impl<'a> TPSReport<'a> {
    pub fn new(stats: &'a AlgorithmStats, alg_type: AlgorithmType, sort: &'a mut Sort) -> Self {
        // Gather algorithm data for each algorithm
        let mut rows = Vec::new();
        match alg_type {
            AlgorithmType::OLL => {
                for (alg, counts) in stats.oll.iter() {
                    if let Some(row) = AlgorithmRow::from_counts(Algorithm::OLL(*alg), counts) {
                        rows.push(row);
                    }
                }
            }
            AlgorithmType::PLL => {
                for (alg, counts) in stats.pll.iter() {
                    if let Some(row) = AlgorithmRow::from_counts(Algorithm::PLL(*alg), counts) {
                        rows.push(row);
                    }
                }
            }
        }

        // Sort algorithms by the desired sort order
        rows.sort_by(|a, b| {
            let result = match sort.column {
                SortColumn::Count => a.count.cmp(&b.count),
                SortColumn::RecognitionTime => {
                    a.recognition_time.partial_cmp(&b.recognition_time).unwrap()
                }
                SortColumn::ExecutionTime => {
                    a.execution_time.partial_cmp(&b.execution_time).unwrap()
                }
                SortColumn::TotalTime => (a.recognition_time + a.execution_time)
                    .partial_cmp(&(b.recognition_time + b.execution_time))
                    .unwrap(),
                SortColumn::MoveCount => a.moves.partial_cmp(&b.moves).unwrap(),
                SortColumn::TPS => a.tps.partial_cmp(&b.tps).unwrap(),
                SortColumn::ExecutionTPS => a.execution_tps.partial_cmp(&b.execution_tps).unwrap(),
            };

            match sort.order {
                SortOrder::Ascending => result,
                SortOrder::Descending => result.reverse(),
            }
        });

        Self { rows, sort }
    }

    pub fn update(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            for row in &self.rows {
                ui.label(format!(
                    "{}: count {} recog {:.2} exec {:.2} total {:.2} moves {:.1} tps {:.2} etps {:.2}",
                    row.algorithm.to_string(),
                    row.count,
                    row.recognition_time,
                    row.execution_time,
                    row.recognition_time + row.execution_time,
                    row.moves,
                    row.tps,
                    row.execution_tps
                ));
            }
        });
    }
}

impl Algorithm {
    fn to_string(&self) -> String {
        match self {
            Algorithm::OLL(oll) => oll.to_string(),
            Algorithm::PLL(pll) => pll.to_str().into(),
        }
    }
}

impl AlgorithmRow {
    fn from_counts(algorithm: Algorithm, counts: &AlgorithmCounts) -> Option<Self> {
        if counts.perform_count < REQUIRED_COUNT {
            // Not enough data to be meaningful for this algorithm
            return None;
        }
        if counts.total_execution_time == 0 {
            // Avoid NaN in any computations, don't include these algorithms as the data
            // won't make sense anyway.
            return None;
        }

        // Compute average stats for this algorithm
        let moves = counts.total_moves as f32 / counts.perform_count as f32;
        let recognition_time =
            counts.total_recognition_time as f32 / 1000.0 / counts.perform_count as f32;
        let execution_time =
            counts.total_execution_time as f32 / 1000.0 / counts.perform_count as f32;
        let total_time = recognition_time + execution_time;
        Some(Self {
            algorithm,
            count: counts.perform_count,
            moves,
            recognition_time,
            execution_time,
            tps: moves / total_time,
            execution_tps: moves / execution_time,
        })
    }
}
