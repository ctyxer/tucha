use std::collections::BTreeMap;

use eframe::egui::{self, Context, Grid, Layout};

use crate::{enums::NewProcess, types::File, ui::window::Window};

#[derive(Clone)]
pub struct Cloud {
    pub clients_files: BTreeMap<String, Vec<File>>,
}

impl Cloud {
    pub fn new() -> Self {
        Self {
            clients_files: BTreeMap::new(),
        }
    }

    pub fn ui(window: &mut Window, ctx: &Context) {
        window.header(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Upload file").clicked() {
                    if let Some(transferred_files) = rfd::FileDialog::new().pick_files() {
                        NewProcess::UploadFiles(transferred_files).start(window);
                    }
                }

                if ui.button("Refresh").clicked() {
                    NewProcess::GetUploadedFiles.start(window);
                }
            });

            if let Some(files) = window
                .cloud_tab
                .clients_files
                .clone()
                .get(&window.current_client)
                .clone()
            {
                Grid::new("Cloud")
                    .num_columns(3)
                    .striped(true)
                    .max_col_width(ui.available_width())
                    .show(ui, |ui| {
                        for file in files {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}{}", &file.metadata.path, &file.name));
                                ui.with_layout(
                                    Layout::right_to_left(eframe::egui::Align::Max),
                                    |ui| {
                                        if ui.button("Download").clicked() {
                                            NewProcess::DownloadFiles(vec![file.message_id])
                                                .start(window);
                                        }
                                        if ui.button("Delete").clicked() {
                                            NewProcess::DeleteFiles(vec![file.message_id])
                                                .start(window);
                                        }
                                    },
                                );
                            });
                            ui.end_row();
                        }
                    });
            }
        });
        window.footer(ctx);
    }
}
