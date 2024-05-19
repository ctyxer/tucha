use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::{mpsc::Sender, Arc},
};

use grammers_client::{
    types::{Chat, Dialog, IterBuffer, LoginToken, Media, User},
    Client as TGClient, Config, InputMessage,
};
use grammers_session::Session;
use grammers_tl_types as tl;
use home::home_dir;
use tl::{
    enums,
    functions::messages::{CreateChat, GetDialogs},
    types::InputUser,
};

use crate::{enums::process::result::ProcessResult, utils::get_secret_data};

use super::{file::metadata::FileMetadata, file::File};

#[derive(Clone)]
pub struct Client {
    pub tg_client: TGClient,
    pub tg_chat: Option<Chat>,
}

impl Client {
    pub fn new(tg_client: TGClient, tg_chat: Option<Chat>) -> Self {
        Self { tg_client, tg_chat }
    }

    async fn find_chat_with_files(
        mut iter_dialogs: IterBuffer<GetDialogs, Dialog>,
        user: &User,
    ) -> Option<Chat> {
        while let Some(dialog) = iter_dialogs.next().await.unwrap() {
            if dialog
                .chat()
                .name()
                .contains(&format!("TuchaCloud-{}", user.id()))
            {
                return Some(dialog.chat().clone());
            }
        }
        None
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

            let chat = Self::find_chat_with_files(tg_client.iter_dialogs(), &user)
                .await
                .unwrap();

            clients.insert(
                user.username().unwrap().to_string(),
                Self::new(tg_client, Some(chat)),
            );
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
                Client::new(tg_client, None),
            ))
            .unwrap();
    }

    pub async fn sign_in_code(
        self: Self,
        sender: Sender<ProcessResult>,
        received_code: String,
        login_token: Arc<LoginToken>,
    ) {
        let tg_client = self.tg_client;

        let user = tg_client
            .sign_in(&login_token, &received_code)
            .await
            .unwrap();

        let path = format!("sessions/{}.session", user.id());
        let _ = fs::write(&path, "");
        let _ = tg_client.session().save_to_file(path);

        if Self::find_chat_with_files(tg_client.iter_dialogs(), &user)
            .await
            .is_none()
        {
            tg_client
                .invoke(&CreateChat {
                    users: vec![enums::InputUser::User(InputUser {
                        user_id: user.id(),
                        access_hash: user.pack().access_hash.unwrap(),
                    })],
                    title: format!("TuchaCloud-{}", user.id()),
                    ttl_period: None,
                })
                .await
                .unwrap();
        }

        let chat = Self::find_chat_with_files(tg_client.iter_dialogs(), &user)
            .await
            .unwrap();

        sender
            .send(ProcessResult::LoggedInWithCode(
                Client::new(tg_client, Some(chat)),
                user.username().unwrap().to_string(),
            ))
            .unwrap();
    }

    pub async fn upload_files(
        self: Self,
        sender: Sender<ProcessResult>,
        transferred_files: Vec<PathBuf>,
    ) {
        let tg_client = self.tg_client;
        let tg_chat = self.tg_chat;
        let creator = tg_client.get_me().await.unwrap();

        for file in transferred_files {
            let file_metadata = FileMetadata::new(String::from(""), &creator);

            let message = InputMessage::text(serde_json::to_string(&file_metadata).unwrap())
                .document(tg_client.upload_file(file.as_os_str()).await.unwrap());

            let _ = &tg_client
                .send_message(&tg_chat.clone().unwrap(), message)
                .await;
        }

        sender.send(ProcessResult::FilesUploaded).unwrap();
    }

    pub async fn get_uploaded_files(self, sender: Sender<ProcessResult>) {
        let tg_client = self.tg_client;
        let tg_chat = self.tg_chat.unwrap();

        let mut files = Vec::<File>::new();

        let mut messages = tg_client.iter_messages(&tg_chat.clone());

        while let Some(message) = messages.next().await.unwrap() {
            if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                let media_enum = message.media().unwrap();
                if let Media::Document(media) = media_enum {
                    let file = File::new(file_metadata, message.id(), media.name().to_string());

                    files.push(file);
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

    pub async fn download_files(self, sender: Sender<ProcessResult>, message_ids: Vec<i32>) {
        let tg_client = self.tg_client;
        let user = tg_client.get_me().await.unwrap();
        let chat = Self::find_chat_with_files(tg_client.iter_dialogs(), &user)
            .await
            .unwrap();

        let messages = tg_client
            .get_messages_by_id(&chat, &message_ids)
            .await
            .unwrap();

        for message in messages {
            match message.clone().unwrap().media().unwrap() {
                Media::Document(document) => {
                    message
                        .unwrap()
                        .download_media(format!("{}/Downloads/{}", home_dir().unwrap().display().to_string(), document.name()))
                        .await
                        .unwrap();
                }
                _ => {}
            }
        }

        sender.send(ProcessResult::FilesDownloaded).unwrap();
    }
}
