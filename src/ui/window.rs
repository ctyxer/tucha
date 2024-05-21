use std::{
    collections::BTreeMap,
    sync::mpsc::{self, Receiver, Sender},
};

use eframe::egui::{self, Color32, ComboBox, Layout, RichText, Spinner};

use crate::{
    enums::process::{current::CurrentProcess, new::NewProcess, result::ProcessResult},
    types::client::Client,
};

use super::tab::{cloud::Cloud, new_session::NewSession, Tab};

pub struct Window {
    pub sender: Sender<ProcessResult>,
    pub receiver: Receiver<ProcessResult>,
    pub clients: std::collections::BTreeMap<String, Client>,
    pub current_client: String,
    pub tab: Tab,
    pub current_process: CurrentProcess,
    pub new_session_tab: NewSession,
    pub cloud_tab: Cloud,
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ProcessResult::check_result(self);
        egui::TopBottomPanel::top("tab").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Cloud, "Cloud");
                ui.selectable_value(&mut self.tab, Tab::NewSession, "New session");

                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    if self.clients.len() > 0 {
                        ComboBox::from_id_source("current-client")
                            .selected_text(format!("{}", self.current_client))
                            .show_ui(ui, |ui| {
                                let mut changed = false;
                                for client in &self.clients {
                                    changed = ui
                                        .selectable_value(
                                            &mut self.current_client,
                                            client.0.to_string(),
                                            client.0,
                                        )
                                        .changed();
                                }
                                if changed {
                                    NewProcess::start(self, NewProcess::GetUploadedFiles);
                                }
                            });
                        if ui.button("Restart clients").clicked() {
                            self.clients.clear();
                            self.cloud_tab.clients_files.clear();
                            NewProcess::start(self, NewProcess::ConnectToAllSavedClients);
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match &self.tab {
            Tab::NewSession => NewSession::ui(self, ui),
            Tab::Cloud => Cloud::ui(self, ui),
        });

        if !matches!(self.current_process, CurrentProcess::Idle) {
            egui::TopBottomPanel::bottom("process").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let CurrentProcess::Error(error) = &self.current_process {
                        ui.label(
                            RichText::new(&error.to_string())
                                .color(Color32::RED)
                                .strong(),
                        );
                        ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                            if ui.button("Close").clicked() {
                                self.current_process = CurrentProcess::Idle;
                            }
                        });
                    } else {
                        ui.add(Spinner::new());
                        ui.label(self.current_process.to_string());
                    }
                });
            });
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();

        let mut window = Self {
            sender,
            receiver,
            clients: BTreeMap::new(),
            current_client: String::new(),
            tab: Tab::Cloud,
            current_process: CurrentProcess::Idle,
            new_session_tab: NewSession::new(),
            cloud_tab: Cloud::new(),
        };

        NewProcess::start(&mut window, NewProcess::ConnectToAllSavedClients);

        window
    }
}
