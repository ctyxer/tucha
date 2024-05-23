use std::{
    collections::BTreeMap,
    sync::mpsc::{self, Receiver, Sender},
};

use eframe::egui::{self, Color32, ComboBox, Layout, RichText, Spinner};

use crate::{enums::*, types::*};

use super::tab::{Cloud, NewSession, Tab};

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

impl Window {
    pub fn get_current_client(&self) -> Result<Client, ProcessError> {
        match self.clients.get(&self.current_client) {
            Some(v) => Ok(v.clone()),
            None => Err(ProcessError::CurrentClientIsNone),
        }
    }

    pub fn header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tab").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Cloud, "Cloud");
                ui.selectable_value(&mut self.tab, Tab::NewSession, "New session");

                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    if self.clients.len() > 0 {
                        ComboBox::from_id_source("current-client")
                            .selected_text(format!("{}", self.current_client))
                            .show_ui(ui, |ui| {
                                let mut is_changed = false;
                                for client in &self.clients {
                                    if !is_changed
                                        && ui
                                            .selectable_value(
                                                &mut self.current_client,
                                                client.0.to_string(),
                                                client.0,
                                            )
                                            .changed()
                                    {
                                        is_changed = true;
                                    }
                                }
                                if is_changed {
                                    NewProcess::GetUploadedFiles.start(self);
                                }
                            });
                        if ui.button("Restart clients").clicked() {
                            self.clients.clear();
                            self.cloud_tab.clients_files.clear();
                            NewProcess::ConnectToAllSavedClients.start(self);
                        }
                    }
                });
            });
        });
    }

    pub fn footer(&mut self, ctx: &egui::Context) {
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

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ProcessResult::check_result(self);
        match &self.tab {
            Tab::NewSession => NewSession::ui(self, ctx),
            Tab::Cloud => Cloud::ui(self, ctx),
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

        NewProcess::ConnectToAllSavedClients.start(&mut window);

        window
    }
}
