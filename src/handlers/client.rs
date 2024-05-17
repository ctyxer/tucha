use std::{
    collections::BTreeMap,
    fs,
    sync::{mpsc::Sender, Arc},
};

use grammers_client::{types::LoginToken, Client as TGClient, Config, InputMessage};
use grammers_session::Session;

use crate::{handlers::process::result::ProcessResult, utils::get_secret_data};

use super::file::File;

#[derive(Clone)]
pub struct Client {
    pub tg_client: TGClient,
}

impl Client {
    pub fn new(tg_client: TGClient) -> Self {
        Self { tg_client }
    }

    pub async fn connect_to_saved_sessions(sender: Sender<ProcessResult>) {
        let session_files = fs::read_dir("./sessions").unwrap();
        let mut clients: BTreeMap<String, Self> = BTreeMap::new();

        for session_file in session_files {
            let path = session_file.unwrap().path();
            let secret_data = get_secret_data().unwrap();

            let tg_client = TGClient::connect(Config {
                session: Session::load_file(path).unwrap(),
                api_id: secret_data.0,
                api_hash: secret_data.1,
                params: Default::default(),
            })
            .await
            .unwrap();

            let user = tg_client.get_me().await.unwrap();

            clients.insert(user.username().unwrap().to_string(), Self::new(tg_client));
        }
        let _ = sender.send(ProcessResult::ConnectedToSavedClients(clients));
    }

    pub async fn send_login_code(sender: Sender<ProcessResult>, phone_number: String) {
        let secret_data = get_secret_data().unwrap();

        let tg_client = TGClient::connect(Config {
            session: Session::new(),
            api_id: secret_data.0,
            api_hash: secret_data.1,
            params: Default::default(),
        })
        .await
        .unwrap();

        let login_token = tg_client.request_login_code(&phone_number).await.unwrap();
        sender
            .send(ProcessResult::LoginCodeSended(
                login_token,
                Client::new(tg_client),
            ))
            .unwrap();
    }

    pub async fn sign_in_code(
        self: Self,
        sender: Sender<ProcessResult>,
        received_code: String,
        login_token: Arc<LoginToken>,
        save_session_file: bool,
    ) {
        let tg_client = self.tg_client;

        let user = tg_client
            .sign_in(&login_token, &received_code)
            .await
            .unwrap();

        if save_session_file {
            let path = format!("sessions/{}.session", user.id());
            let _ = fs::write(&path, "");
            let _ = tg_client.session().save_to_file(path);
        }

        sender
            .send(ProcessResult::LoggedInWithCode(
                Client::new(tg_client),
                user.username().unwrap().to_string(),
            ))
            .unwrap();
    }

    pub async fn upload_files(self: Self, sender: Sender<ProcessResult>, file_paths: Vec<String>) {
        let tg_client = self.tg_client;

        for file_path in file_paths {
            let file = File::new(file_path);

            let message = InputMessage::text(serde_json::to_string(&file).unwrap())
                .document(tg_client.upload_file(file.path).await.unwrap());

            let mut dialogs_iter = tg_client.iter_dialogs();
            while let Some(dialog) = dialogs_iter.next().await.unwrap() {
                if dialog.chat().name().contains("TuchaCloud") {
                    let _ = tg_client.send_message(dialog.chat(), message).await;
                    break;
                }
            }
        }

        sender.send(ProcessResult::FilesUploaded).unwrap();
    }

    pub async fn get_uploaded_files(self, sender: Sender<ProcessResult>) {
        let tg_client = self.tg_client;

        let mut files = Vec::<File>::new();

        let mut dialogs_iter = tg_client.iter_dialogs();
        while let Some(dialog) = dialogs_iter.next().await.unwrap() {
            if dialog.chat().name().contains("TuchaCloud") {
                let mut messages = tg_client.iter_messages(dialog.chat());

                while let Some(message) = messages.next().await.unwrap() {
                    if let Ok(file) = serde_json::from_str::<File>(message.text()) {
                        files.push(file);
                    }
                }
            }
        }

        sender
            .send(ProcessResult::UploadedFilesReceived(
                tg_client
                    .get_me()
                    .await
                    .unwrap()
                    .username()
                    .unwrap()
                    .to_string(),
                files,
            ))
            .unwrap();
    }
}
