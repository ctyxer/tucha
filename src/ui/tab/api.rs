use eframe::egui::{self, Button, Context, Grid};

use crate::{enums::process::new::NewProcess, ui::window::Window};

pub struct API {
    pub api_id: String,
    pub api_hash: String,
}

impl API {
    pub fn new() -> Self {
        Self {
            api_hash: String::new(),
            api_id: String::new(),
        }
    }

    pub fn ui(window: &mut Window, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Grid::new("API").num_columns(2).show(ui, |ui| {
                ui.label("Telegram API link:");
                ui.hyperlink_to(
                    "https://my.telegram.org/apps",
                    "https://my.telegram.org/apps",
                );
                ui.end_row();

                ui.label("api_id: ");
                ui.text_edit_singleline(&mut window.api_tab.api_id);
                ui.end_row();

                ui.label("api_hash: ");
                ui.text_edit_singleline(&mut window.api_tab.api_hash);
                ui.end_row();

                if ui
                    .add_enabled(
                        !window.api_tab.api_hash.is_empty() && !window.api_tab.api_id.is_empty(),
                        Button::new("Save"),
                    )
                    .clicked()
                {
                    NewProcess::StoreAPIKeysInFile.start(window);
                };
            });
        });
    }
}
