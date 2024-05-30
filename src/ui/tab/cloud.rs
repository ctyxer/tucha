use std::{collections::BTreeMap, path::PathBuf};

use eframe::egui::{self, Context, Grid, Label, Layout, TextEdit};

use crate::{enums::NewProcess, types::Dir, ui::window::Window};

#[derive(Clone)]
pub struct Cloud {
    pub clients_roots: BTreeMap<String, Dir>,
    pub current_path: PathBuf,
    is_creating_folder: bool,
    new_dir_name: String,
}

impl Cloud {
    pub fn new() -> Self {
        Self {
            clients_roots: BTreeMap::new(),
            current_path: PathBuf::from("/"),
            is_creating_folder: false,
            new_dir_name: String::new(),
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

            ui.add(Label::new(format!(
                "Current path: {}",
                &window.cloud_tab.current_path.display().to_string()
            )))
            .highlight();

            if ui.button("Create directory").clicked() {
                window.cloud_tab.is_creating_folder = true;
            }
            if window.cloud_tab.is_creating_folder {
                ui.horizontal(|ui| {
                    ui.label("Directory name: ");

                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui.button("Cancel").clicked() {
                            window.cloud_tab.is_creating_folder = false;
                        }
                        if ui.button("Create").clicked() {
                            window
                                .cloud_tab
                                .current_path
                                .push(window.cloud_tab.new_dir_name.clone());
                            window.cloud_tab.new_dir_name.clear();
                        }
                        ui.add(
                            TextEdit::singleline(&mut window.cloud_tab.new_dir_name)
                                .min_size(ui.available_size()),
                        );
                    });
                });
            }

            ui.separator();

            if let Some(root) = window
                .cloud_tab
                .clients_roots
                .clone()
                .get_mut(&window.current_client)
            {
                Grid::new("Cloud")
                    .num_columns(3)
                    .striped(true)
                    .max_col_width(ui.available_width())
                    .show(ui, |ui| {
                        if window.cloud_tab.current_path != PathBuf::from("/") {
                            if ui.add(Label::new("..")).clicked() {
                                window.cloud_tab.current_path.pop();
                            }
                            ui.end_row();
                        }

                        if let Some(relative_dir) =
                            root.find_directory_by_relative_path(&window.cloud_tab.current_path)
                        {
                            for dir in relative_dir.get_children_dirs().values() {
                                ui.horizontal(|ui| {
                                    if ui.add(Label::new(&dir.name)).clicked() {
                                        window.cloud_tab.current_path.push(&dir.name);
                                    }
                                    ui.with_layout(
                                        Layout::right_to_left(eframe::egui::Align::Max),
                                        |ui| {
                                            if ui.button("Download").clicked() {
                                                NewProcess::DownloadFiles(
                                                    dir.get_files_messages_ids(),
                                                )
                                                .start(window);
                                            }
                                            if ui.button("Delete").clicked() {
                                                NewProcess::DeleteFiles(
                                                    dir.get_files_messages_ids(),
                                                )
                                                .start(window);
                                            }
                                        },
                                    );
                                });
                                ui.end_row();
                            }

                            for file in &relative_dir.files {
                                ui.horizontal(|ui| {
                                    ui.label(&file.path.display().to_string());
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
                        }
                    });
            }
        });
        window.footer(ctx);
    }
}
