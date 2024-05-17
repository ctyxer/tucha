use std::{
    collections::BTreeMap,
    sync::mpsc::{self, Receiver, Sender},
};

use eframe::egui::{self, ComboBox, Spinner};

use crate::handlers::{
    client::Client,
    process::{current::CurrentProcess, new::NewProcess, result::ProcessResult},
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

                if self.clients.len() > 0 {
                    ComboBox::from_id_source("current-client")
                        .selected_text(format!("{}", self.current_client))
                        .show_ui(ui, |ui| {
                            for client in &self.clients {
                                ui.selectable_value(
                                    &mut self.current_client,
                                    client.0.to_string(),
                                    client.0,
                                );
                            }
                        });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match &self.tab {
            Tab::NewSession => NewSession::ui(self, ui),
            Tab::Cloud => Cloud::ui(self, ui),
        });

        if !matches!(self.current_process, CurrentProcess::Idle) {
            egui::TopBottomPanel::bottom("process").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(Spinner::new());
                    ui.label(self.current_process.to_string());
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
