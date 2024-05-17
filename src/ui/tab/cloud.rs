use std::collections::BTreeMap;

use eframe::egui::Ui;

use crate::{
    handlers::{file::File, process::new::NewProcess},
    ui::window::Window,
};

pub struct Cloud {
    pub uploaded_file_paths: Vec<String>,
    pub clients_files: BTreeMap<String, Vec<File>>,
}

impl Cloud {
    pub fn new() -> Self {
        Self {
            uploaded_file_paths: Vec::new(),
            clients_files: BTreeMap::new(),
        }
    }

    fn file_ui(ui: &mut Ui, file: &File) {
        ui.separator();
        ui.label(&file.path);
    }

    pub fn ui(window: &mut Window, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Upload file").clicked() {
                let file = rfd::FileDialog::new().pick_file();
                window
                    .cloud_tab
                    .uploaded_file_paths
                    .push(file.unwrap().display().to_string());
    
                NewProcess::start(window, NewProcess::UploadFiles);
            }

            if ui.button("Refresh").clicked() {    
                NewProcess::start(window, NewProcess::GetUploadedFiles);
            }
        });

        if let Some(files) = window.cloud_tab.clients_files.get(&window.current_client) {
            for file in files {
                Self::file_ui(ui, file);
            }
        }
    }
}
