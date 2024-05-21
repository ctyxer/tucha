use std::{collections::BTreeMap, sync::Arc};

use grammers_client::types::LoginToken;

use crate::{
    types::{client::Client, file::File},
    ui::{tab::Tab, window::Window},
};

use super::{current::CurrentProcess, error::ProcessError, new::NewProcess};

pub enum ProcessResult {
    Error(ProcessError),
    ConnectedToSavedClients(BTreeMap<String, Client>),
    LoginCodeSended(LoginToken, Client),
    LoggedIn(Client, String),
    FilesUploaded,
    UploadedFilesReceived(String, Vec<File>),
    FilesDownloaded,
}

impl ProcessResult {
    pub fn check_result(window: &mut Window) {
        let receiver = &mut window.receiver;
        if let Ok(process_result) = receiver.try_recv() {
            match process_result {
                ProcessResult::ConnectedToSavedClients(clients) => {
                    window.current_process = CurrentProcess::Idle;

                    if let Some(first_client) = &clients.clone().first_key_value() {
                        window.clients = clients;
                        window.current_client = first_client.0.to_string();

                        NewProcess::start(window, NewProcess::GetUploadedFiles);
                    }
                }
                ProcessResult::LoginCodeSended(login_token, client) => {
                    window.current_process = CurrentProcess::Idle;
                    window.new_session_tab.login_token = Some(Arc::new(login_token));
                    window.new_session_tab.is_code_received = true;
                    window.new_session_tab.incomplete_client = Some(client);
                }
                ProcessResult::LoggedIn(client, client_name) => {
                    window.current_process = CurrentProcess::Idle;
                    window.tab = Tab::Cloud;

                    window.current_client = client_name.clone();

                    window.clients.insert(client_name, client);
                }
                ProcessResult::FilesUploaded => {
                    window.current_process = CurrentProcess::Idle;

                    NewProcess::start(window, NewProcess::GetUploadedFiles);
                }
                ProcessResult::UploadedFilesReceived(client_name, files) => {
                    window.current_process = CurrentProcess::Idle;

                    window.cloud_tab.clients_files.insert(client_name, files);
                }
                ProcessResult::FilesDownloaded => {
                    window.current_process = CurrentProcess::Idle;
                }
                ProcessResult::Error(error) => {
                    window.current_process = CurrentProcess::Error(error);
                }
            }
        }
    }
}
