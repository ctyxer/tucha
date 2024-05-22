use std::sync::Arc;

use eframe::egui::{self, Button, Context, Grid, Layout, TextEdit};
use grammers_client::types::LoginToken;

use crate::{enums::process::new::NewProcess, types::client::Client, ui::window::Window};

pub struct NewSession {
    pub new_session_name: String,
    pub phone_number: String,
    pub reveived_code: String,
    pub user_password: String,
    pub is_code_received: bool,
    pub login_token: Option<Arc<LoginToken>>,
    pub incomplete_client: Option<Client>,
}

impl NewSession {
    pub fn new() -> Self {
        Self {
            new_session_name: String::new(),
            phone_number: String::new(),
            reveived_code: String::new(),
            user_password: String::new(),
            is_code_received: false,
            login_token: None,
            incomplete_client: None,
        }
    }

    pub fn ui(window: &mut Window, ctx: &Context) {
        window.header(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            Grid::new("New session").num_columns(2).show(ui, |ui| {
                ui.label("Phone number: ");
                ui.add_enabled(
                    !window.new_session_tab.is_code_received,
                    TextEdit::singleline(&mut window.new_session_tab.phone_number)
                        .min_size(ui.available_size()),
                );
                ui.end_row();

                ui.label("Password (if required): ");
                ui.add(
                    TextEdit::singleline(&mut window.new_session_tab.user_password)
                        .min_size(ui.available_size()),
                );
                ui.end_row();

                ui.label("Received code: ");
                ui.with_layout(Layout::right_to_left(eframe::egui::Align::Min), |ui| {
                    if ui.button("Send code").clicked() {
                        NewProcess::SendLoginCode.start(window);
                    };

                    let received_code_singleline =
                        TextEdit::singleline(&mut window.new_session_tab.reveived_code)
                            .min_size(ui.available_size());

                    ui.add_enabled(
                        window.new_session_tab.is_code_received,
                        received_code_singleline,
                    );
                });
                ui.end_row();

                let sign_in_button = Button::new("Sign in");
                if ui
                    .add_enabled(
                        window.new_session_tab.is_code_received
                            && window.new_session_tab.reveived_code.len() >= 5,
                        sign_in_button,
                    )
                    .clicked()
                {
                    NewProcess::SingIn.start(window);
                };
                ui.end_row();
            });
        });
        window.footer(ctx);
    }
}
