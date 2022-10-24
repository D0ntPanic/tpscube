use super::{Algorithm, AlgorithmRender, AlgorithmType};
use crate::font::{FontSize, LabelFontSize};
use crate::theme::Theme;
use egui::{Label, ScrollArea, Stroke, Ui};
use tpscube_core::{ExtendedMove, KnownAlgorithms, OLLAlgorithm, PLLAlgorithm};

pub(super) struct AlgorithmList {
    rows: Vec<AlgorithmRow>,
}

struct AlgorithmRow {
    algorithm: Algorithm,
    moves: Vec<ExtendedMove>,
}

impl AlgorithmList {
    pub fn new(alg_type: AlgorithmType) -> Self {
        let mut rows = Vec::new();
        match alg_type {
            AlgorithmType::OLL => {
                for alg in OLLAlgorithm::all() {
                    let moves = KnownAlgorithms::oll(*alg)[0].clone();
                    rows.push(AlgorithmRow {
                        algorithm: Algorithm::OLL(*alg),
                        moves,
                    });
                }
            }
            AlgorithmType::PLL => {
                for alg in PLLAlgorithm::all() {
                    let moves = KnownAlgorithms::pll(*alg)[0].clone();
                    rows.push(AlgorithmRow {
                        algorithm: Algorithm::PLL(*alg),
                        moves,
                    });
                }
            }
        }

        Self { rows }
    }

    pub fn update(&self, ui: &mut Ui) {
        ui.visuals_mut().widgets.inactive.bg_fill = Theme::BackgroundHighlight.into();
        ui.visuals_mut().widgets.hovered.bg_fill = Theme::Disabled.into();
        ui.visuals_mut().widgets.active.bg_fill = Theme::Disabled.into();
        ScrollArea::auto_sized()
            .id_source("algorithm_list")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for row in &self.rows {
                        ui.horizontal(|ui| {
                            row.algorithm.draw(
                                ui,
                                &row.moves,
                                ui.fonts().row_height(FontSize::Normal.into()) * 4.0,
                                true,
                                None,
                            );

                            ui.scope(|ui| {
                                ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                                    width: 1.0,
                                    color: Theme::Light.into(),
                                };
                                ui.separator();
                            });

                            ui.add(
                                Label::new(
                                    row.moves
                                        .iter()
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>()
                                        .join(" "),
                                )
                                .font_size(FontSize::Section)
                                .wrap(true),
                            );
                        });

                        ui.scope(|ui| {
                            ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke {
                                width: 1.0,
                                color: Theme::DarkBlue.into(),
                            };
                            ui.separator();
                        });
                    }
                })
            });
    }
}
