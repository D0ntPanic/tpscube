use crate::font::FontSize;
use crate::style::settings_visuals;
use crate::theme::Theme;
use crate::widgets::CustomWidgets;
use anyhow::Result;
use egui::{containers::ScrollArea, widgets::Label, CentralPanel, CtxRef, Sense, Stroke};
use tpscube_core::{History, SyncRequest};

pub struct SettingsWidget {
    sync_key_visible: bool,
    set_key_visible: bool,
    new_sync_key: String,
    import_result: Option<Result<String>>,
    export_result: Option<Result<()>>,
}

impl SettingsWidget {
    pub fn new() -> Self {
        Self {
            sync_key_visible: false,
            set_key_visible: false,
            new_sync_key: "".into(),
            import_result: None,
            export_result: None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn import_solves_from_path(path: &str, history: &mut History) -> Result<String> {
        let contents = String::from_utf8(std::fs::read(path)?)?;
        history.import(contents)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn import_solves(&mut self, history: &mut History) {
        if let Some(path) = tinyfiledialogs::open_file_dialog(
            "Import Solves",
            ".",
            Some((&["*.json", "*.csv", "*.txt"], "Solve backups")),
        ) {
            self.import_result = Some(Self::import_solves_from_path(&path, history));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn export_solves_to_path(path: &str, history: &mut History) -> Result<()> {
        let contents = history.export()?;
        Ok(std::fs::write(path, contents.as_bytes())?)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn export_solves(&mut self, history: &mut History) {
        if let Some(path) = tinyfiledialogs::save_file_dialog_with_filter(
            "Export Solves",
            "tpscube.json",
            &["*.json"],
            "Solve backups",
        ) {
            self.export_result = Some(Self::export_solves_to_path(&path, history));
        }
    }

    pub fn update(&mut self, ctxt: &CtxRef, _frame: &mut epi::Frame<'_>, history: &mut History) {
        ctxt.set_visuals(settings_visuals());
        CentralPanel::default().show(ctxt, |ui| {
            ui.visuals_mut().widgets.inactive.bg_fill = Theme::BackgroundHighlight.into();
            ui.visuals_mut().widgets.hovered.bg_fill = Theme::Disabled.into();
            ui.visuals_mut().widgets.active.bg_fill = Theme::Disabled.into();
            ScrollArea::auto_sized().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.section("Cloud Sync");

                    // Show sync key option
                    if ui
                        .add(
                            Label::new(format!(
                                "👁  {} sync key",
                                if self.sync_key_visible {
                                    "Hide"
                                } else {
                                    "Show"
                                }
                            ))
                            .text_style(FontSize::Section.into())
                            .sense(Sense::click()),
                        )
                        .clicked()
                    {
                        self.sync_key_visible = !self.sync_key_visible
                    }
                    if self.sync_key_visible {
                        ui.add(
                            Label::new(history.sync_key())
                                .text_style(FontSize::Scramble.into())
                                .text_color(Theme::Yellow),
                        );
                        if ui
                            .add(
                                Label::new("🗐  Copy")
                                    .text_style(FontSize::Section.into())
                                    .sense(Sense::click()),
                            )
                            .clicked()
                        {
                            ui.output().copied_text = history.sync_key().into();
                        }
                    }
                    ui.add(
                        Label::new(format!(
                            "{} your active sync key. This key acts like a password to sync your \
                                solve information across devices. Never share your sync key with \
                                anyone.",
                            if self.sync_key_visible {
                                "This is"
                            } else {
                                "Show"
                            }
                        ))
                        .wrap(true),
                    );

                    ui.add_space(8.0);

                    // Set sync key option
                    if ui
                        .add(
                            Label::new("🗝  Set sync key")
                                .text_style(FontSize::Section.into())
                                .sense(Sense::click()),
                        )
                        .clicked()
                    {
                        self.set_key_visible = !self.set_key_visible;
                    }
                    ui.add(
                        Label::new(
                            "If you already have other devices with solve information, you can \
                                set your sync key here to sync with them. You can view \
                                your sync key on any device that is already being synced.",
                        )
                        .wrap(true),
                    );

                    if self.set_key_visible {
                        // If set sync key is active, show edit box
                        ui.add_space(8.0);
                        ui.add(Label::new("New sync key: ").text_color(Theme::Yellow));
                        ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke {
                            width: 1.0,
                            color: Theme::Disabled.into(),
                        };
                        ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke {
                            width: 1.0,
                            color: Theme::Disabled.into(),
                        };
                        ui.style_mut().visuals.widgets.active.bg_stroke = Stroke {
                            width: 1.0,
                            color: Theme::Content.into(),
                        };
                        ui.text_edit_singleline(&mut self.new_sync_key);

                        // Validate the sync key being entered
                        if let Some(key) = SyncRequest::validate_sync_key(&self.new_sync_key) {
                            if key == history.sync_key() {
                                ui.add(Label::new("Key set").text_color(Theme::Green));
                            } else {
                                // Sync key is valid and different, allow the user to set it
                                if ui
                                    .add(
                                        Label::new("✔  Save")
                                            .text_style(FontSize::Section.into())
                                            .sense(Sense::click()),
                                    )
                                    .clicked()
                                {
                                    let _ = history.set_sync_key(&key);
                                }
                            }
                        } else {
                            // Sync key is not valid, show error
                            ui.add(Label::new("(Not valid)").text_color(Theme::Red));
                        }
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        ui.add_space(16.0);

                        ui.section("Import / Export");

                        // Import solves option
                        if ui
                            .add(
                                Label::new("🗁  Import solves")
                                    .text_style(FontSize::Section.into())
                                    .sense(Sense::click()),
                            )
                            .clicked()
                        {
                            self.import_solves(history);
                        }
                        if let Some(result) = &self.import_result {
                            match result {
                                Ok(message) => {
                                    ui.add(
                                        Label::new(format!("Import complete.\n{}", message))
                                            .text_color(Theme::Green),
                                    );
                                }
                                Err(error) => {
                                    ui.add(
                                        Label::new(format!("Error: {}", error))
                                            .wrap(true)
                                            .text_color(Theme::Red),
                                    );
                                }
                            }
                        }
                        ui.add(
                            Label::new(
                                "Import solves from a backup. Supports backups from \
                               TPS Cube, csTimer, and Cubeast.",
                            )
                            .wrap(true),
                        );

                        ui.add_space(8.0);

                        // Export solves option
                        if ui
                            .add(
                                Label::new("🗐  Export solves")
                                    .text_style(FontSize::Section.into())
                                    .sense(Sense::click()),
                            )
                            .clicked()
                        {
                            self.export_solves(history);
                        }
                        if let Some(result) = &self.export_result {
                            match result {
                                Ok(()) => {
                                    ui.add(Label::new("Export complete.").text_color(Theme::Green));
                                }
                                Err(error) => {
                                    ui.add(
                                        Label::new(format!("Error: {}", error))
                                            .wrap(true)
                                            .text_color(Theme::Red),
                                    );
                                }
                            }
                        }
                        ui.label("Export all solve information to a file for backup.")
                    }
                });
            });
        });
    }
}
