use std::{collections::BTreeMap, sync::Arc};

use grammers_client::types::LoginToken;

use crate::{
    types::{Client, Dir, File},
    ui::{tab::Tab, window::Window},
};

use super::{CurrentProcess, NewProcess, ProcessError};

pub enum ProcessResult {
    Error(ProcessError),
    ConnectedToSavedClients(BTreeMap<String, Client>),
    LoginCodeSended(LoginToken, Client),
    LoggedIn(Client, String),
    FilesUploaded,
    UploadedFilesReceived(String, Vec<File>),
    FilesDownloaded,
    FilesDeleted,
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

                        NewProcess::GetUploadedFiles.start(window);
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

                    NewProcess::GetUploadedFiles.start(window);
                }
                ProcessResult::UploadedFilesReceived(client_name, files) => {
                    window.current_process = CurrentProcess::Idle;
                    let mut root = Dir::root();

                    for file in files {
                        let parent = file.path.parent();
                        let components = parent.components().into_iter();

                        if components.clone().count() > 0 {
                            let new_dir = root.add_new_path(components);
                            new_dir.files.push(file);
                        } else {
                            root.files.push(file);
                        }
                    }

                    window.cloud_tab.clients_roots.insert(client_name, root);
                }
                ProcessResult::FilesDownloaded => {
                    window.current_process = CurrentProcess::Idle;
                }
                ProcessResult::Error(error) => {
                    window.current_process = CurrentProcess::Error(error);
                }
                ProcessResult::FilesDeleted => {
                    window.current_process = CurrentProcess::Idle;

                    NewProcess::GetUploadedFiles.start(window);
                }
            }
        }
    }
}
