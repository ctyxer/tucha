use std::collections::BTreeMap;

use eframe::egui::{Grid, Layout, Ui};

use crate::{enums::process::new::NewProcess, types::file::File, ui::window::Window};

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

    pub fn ui(window: &mut Window, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Upload file").clicked() {
                if let Some(transferred_files) = rfd::FileDialog::new().pick_files() {
                    NewProcess::start(window, NewProcess::UploadFiles(transferred_files));
                }
            }

            if ui.button("Refresh").clicked() {
                NewProcess::start(window, NewProcess::GetUploadedFiles);
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
                .num_columns(2)
                .striped(true)
                .max_col_width(ui.available_width())
                .show(ui, |ui| {
                    for file in files {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}{}", &file.metadata.path, &file.name));
                            ui.with_layout(Layout::right_to_left(eframe::egui::Align::Max), |ui| {
                                if ui.button("Download").clicked() {
                                    NewProcess::start(
                                        window,
                                        NewProcess::DownloadFiles(vec![file.message_id]),
                                    );
                                }
                            });
                        });
                        ui.end_row();
                    }
                });
        }
    }
}
