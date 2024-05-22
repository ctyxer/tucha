use std::path::PathBuf;
use std::sync::mpsc::Sender;

use crate::types::api_keys::APIKeys;
use crate::types::client::Client;
use crate::ui::window::Window;

use crate::enums::process::current::CurrentProcess;

use super::error::ProcessError;
use super::result::ProcessResult;

pub enum NewProcess {
    StoreAPIKeysInFile,
    ConnectToAllSavedClients,
    GetUploadedFiles,
    SendLoginCode,
    SingIn,
    UploadFiles(Vec<PathBuf>),
    DownloadFiles(Vec<i32>),
}

impl NewProcess {
    fn send_result(
        sender: Sender<ProcessResult>,
        task_result: Result<ProcessResult, ProcessError>,
    ) {
        match task_result {
            Ok(v) => {
                let _ = sender.send(v);
            }
            Err(e) => {
                let _ = sender.send(ProcessResult::Error(e));
            }
        }
    }

    fn store_api(api_id: String, api_hash: String) -> Result<ProcessResult, ProcessError> {
        APIKeys::new(
            api_id
                .parse::<i32>()
                .map_err(|_| ProcessError::CannotParseAPIIDToInteger)?,
            api_hash.clone(),
        )
        .save()
        .map_err(|_| ProcessError::CannotSaveAPIKeysInFile)?;
        Ok(ProcessResult::StoredAPIKeys)
    }

    pub fn start(window: &mut Window, new_process: NewProcess) {
        match &new_process {
            NewProcess::StoreAPIKeysInFile => {
                Self::send_result(
                    window.sender.clone(),
                    Self::store_api(
                        window.api_tab.api_id.clone(),
                        window.api_tab.api_hash.clone(),
                    ),
                );
            }
            NewProcess::ConnectToAllSavedClients => {
                window.current_process = CurrentProcess::ConnectingToAllSavedClients;
                let sender = window.sender.clone();

                tokio::spawn(async move {
                    Self::send_result(sender, Client::connect_to_saved_sessions().await);
                });
            }
            NewProcess::SendLoginCode => {
                window.current_process = CurrentProcess::SendingLoginCode;

                let sender = window.sender.clone();
                let phone_number = window.new_session_tab.phone_number.clone();

                tokio::spawn(async move {
                    Self::send_result(sender, Client::send_login_code(phone_number).await);
                });
            }
            NewProcess::SingIn => {
                window.current_process = CurrentProcess::LogInWithCode;

                let sender = window.sender.clone();
                let incomplete_client = match window.new_session_tab.incomplete_client.clone() {
                    Some(v) => v,
                    None => {
                        let _sender_result =
                            sender.send(ProcessResult::Error(ProcessError::IncompleteClientIsNone));
                        return;
                    }
                };
                let reveived_code = window.new_session_tab.reveived_code.clone();
                let login_token = match window.new_session_tab.login_token.clone() {
                    Some(v) => v,
                    None => {
                        let _sender_result =
                            sender.send(ProcessResult::Error(ProcessError::LoginTokenIsNone));
                        return;
                    }
                };
                let user_password = window.new_session_tab.user_password.clone();

                tokio::spawn(async move {
                    Self::send_result(
                        sender,
                        incomplete_client
                            .sign_in(reveived_code, login_token, user_password)
                            .await,
                    );
                });
            }
            NewProcess::UploadFiles(transferred_files) => {
                window.current_process = CurrentProcess::UploadingFiles;

                let sender = window.sender.clone();
                let transferred_files = transferred_files.clone();
                let client = match window.clients.get(&window.current_client) {
                    Some(v) => v,
                    None => {
                        let _sender_result =
                            sender.send(ProcessResult::Error(ProcessError::CurrentClientIsNone));
                        return;
                    }
                }
                .clone();

                tokio::spawn(async move {
                    Self::send_result(sender, client.upload_files(transferred_files.clone()).await);
                });
            }
            NewProcess::GetUploadedFiles => {
                window.current_process = CurrentProcess::GettingUploadedFiles;

                let sender = window.sender.clone();
                let client = match window.clients.get(&window.current_client) {
                    Some(v) => v,
                    None => {
                        let _sender_result =
                            sender.send(ProcessResult::Error(ProcessError::CurrentClientIsNone));
                        return;
                    }
                }
                .clone();

                tokio::spawn(async move {
                    Self::send_result(sender, client.get_uploaded_files().await);
                });
            }
            NewProcess::DownloadFiles(message_ids) => {
                window.current_process = CurrentProcess::DownloadingFiles;

                let sender = window.sender.clone();
                let client = match window.clients.get(&window.current_client) {
                    Some(v) => v,
                    None => {
                        let _sender_result =
                            sender.send(ProcessResult::Error(ProcessError::CurrentClientIsNone));
                        return;
                    }
                }
                .clone();
                let message_ids = message_ids.clone();

                tokio::spawn(async move {
                    Self::send_result(sender, client.download_files(message_ids.clone()).await);
                });
            }
        };
    }
}
