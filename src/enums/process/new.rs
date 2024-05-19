use std::path::PathBuf;

use crate::types::client::Client;
use crate::ui::window::Window;

use crate::enums::process::current::CurrentProcess;

pub enum NewProcess {
    ConnectToAllSavedClients,
    GetUploadedFiles,
    SendLoginCode,
    SingInToken,
    UploadFiles(Vec<PathBuf>),
    DownloadFiles(Vec<i32>),
}

impl NewProcess {
    pub fn start(window: &mut Window, new_process: NewProcess) {
        match &new_process {
            NewProcess::ConnectToAllSavedClients => {
                window.current_process = CurrentProcess::ConnectingToAllSavedClients;
                let sender = window.sender.clone();

                tokio::spawn(Client::connect_to_saved_sessions(sender));
            }
            NewProcess::SendLoginCode => {
                window.current_process = CurrentProcess::SendingLoginCode;

                let sender = window.sender.clone();
                let phone_number = window.new_session_tab.phone_number.clone();

                tokio::spawn(Client::send_login_code(sender, phone_number));
            }
            NewProcess::SingInToken => {
                window.current_process = CurrentProcess::LogInWithCode;

                let sender = window.sender.clone();
                let incomplete_client = window.new_session_tab.incomplete_client.clone().unwrap();
                let reveived_code = window.new_session_tab.reveived_code.clone();
                let login_token = window.new_session_tab.login_token.clone().unwrap();

                tokio::spawn(incomplete_client.sign_in_code(sender, reveived_code, login_token));
            }
            NewProcess::UploadFiles(transferred_files) => {
                window.current_process = CurrentProcess::UploadingFiles;

                let sender = window.sender.clone();
                let client = window.clients.get(&window.current_client).unwrap().clone();

                tokio::spawn(client.upload_files(sender, transferred_files.clone()));
            }
            NewProcess::GetUploadedFiles => {
                window.current_process = CurrentProcess::GettingUploadedFiles;

                let sender = window.sender.clone();
                let client = window.clients.get(&window.current_client).unwrap().clone();

                tokio::spawn(client.get_uploaded_files(sender));
            }
            NewProcess::DownloadFiles(message_ids) => {
                window.current_process = CurrentProcess::DownloadingFiles;

                let sender = window.sender.clone();
                let client = window.clients.get(&window.current_client).unwrap().clone();

                tokio::spawn(client.download_files(sender, message_ids.clone()));
            },
        };
    }
}
