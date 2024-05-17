use crate::handlers::client::Client;
use crate::ui::window::Window;

use crate::handlers::process::current::CurrentProcess;

pub enum NewProcess {
    ConnectToAllSavedClients,
    GetUploadedFiles,
    SendLoginCode,
    SingInToken,
    UploadFiles,
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
                let save_session_file = window.new_session_tab.save_session_file.clone();

                tokio::spawn(incomplete_client.sign_in_code(
                    sender,
                    reveived_code,
                    login_token,
                    save_session_file,
                ));
            }
            NewProcess::UploadFiles => {
                window.current_process = CurrentProcess::UploadingFiles;

                let sender = window.sender.clone();
                let file_paths = window.cloud_tab.uploaded_file_paths.clone();
                let client = window.clients.get(&window.current_client).unwrap().clone();

                tokio::spawn(client.upload_files(sender, file_paths));
            }
            NewProcess::GetUploadedFiles => {
                window.current_process = CurrentProcess::GettingUploadedFiles;

                let sender = window.sender.clone();
                let client = window.clients.get(&window.current_client).unwrap().clone();

                tokio::spawn(client.get_uploaded_files(sender));
            }
        };
    }
}
