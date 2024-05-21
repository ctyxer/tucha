use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};

use grammers_client::{
    types::{Chat, Dialog, IterBuffer, LoginToken, Media, Message, User},
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

use crate::{
    enums::process::{error::ProcessError, result::ProcessResult},
    utils::get_secret_data,
};

use super::{file::metadata::FileMetadata, file::File};

#[derive(Clone, Debug)]
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
    ) -> Result<Chat, ProcessError> {
        while let Some(dialog) = iter_dialogs
            .next()
            .await
            .map_err(|_| ProcessError::CannotGetDialogs)?
        {
            if dialog
                .chat()
                .name()
                .contains(&format!("TuchaCloud-{}", user.id()))
            {
                return Ok(dialog.chat().clone());
            }
        }
        Err(ProcessError::ChatIsNotFound)
    }

    pub async fn connect_to_saved_sessions() -> Result<ProcessResult, ProcessError> {
        let mut session_files =
            fs::read_dir("./sessions").map_err(|_| ProcessError::CannotReadSessionsDirectory)?;
        let mut clients: BTreeMap<String, Self> = BTreeMap::new();

        while let Some(Ok(session_file)) = session_files.next() {
            let path = session_file.path();
            let secret_data = get_secret_data()?;

            let tg_client = TGClient::connect(Config {
                session: Session::load_file(path)
                    .map_err(|_| ProcessError::CannotLoadSessionFile)?,
                api_id: secret_data.0,
                api_hash: secret_data.1,
                params: Default::default(),
            })
            .await
            .map_err(|_| ProcessError::ClientIsNotConnected)?;

            let user = tg_client
                .get_me()
                .await
                .map_err(|_| ProcessError::CannotGetUserData)?;

            let chat = Self::find_chat_with_files(tg_client.iter_dialogs(), &user).await?;

            clients.insert(
                match user.username() {
                    Some(v) => v,
                    None => return Err(ProcessError::UsernameIsNone),
                }
                .to_string(),
                Self::new(tg_client, Some(chat)),
            );
        }
        Ok(ProcessResult::ConnectedToSavedClients(clients))
    }

    pub async fn send_login_code(phone_number: String) -> Result<ProcessResult, ProcessError> {
        let secret_data = get_secret_data()?;

        let tg_client = TGClient::connect(Config {
            session: Session::new(),
            api_id: secret_data.0,
            api_hash: secret_data.1,
            params: Default::default(),
        })
        .await
        .map_err(|_| ProcessError::ClientIsNotConnected)?;

        let login_token = tg_client
            .request_login_code(&phone_number)
            .await
            .map_err(|_| ProcessError::LoginCodeIsNotSended)?;
        Ok(ProcessResult::LoginCodeSended(
            login_token,
            Client::new(tg_client, None),
        ))
    }

    pub async fn sign_in_code(
        self: Self,
        received_code: String,
        login_token: Arc<LoginToken>,
    ) -> Result<ProcessResult, ProcessError> {
        let tg_client = self.tg_client;

        let user = match tg_client.sign_in(&login_token, &received_code).await {
            Ok(v) => Ok(v),
            Err(err) => match err {
                grammers_client::SignInError::SignUpRequired {
                    terms_of_service: _,
                } => Err(ProcessError::OtherSignInError),
                grammers_client::SignInError::PasswordRequired(v) => {
                    Err(ProcessError::PasswordRequired(v))
                }
                grammers_client::SignInError::InvalidCode => Err(ProcessError::InvalidCode),
                grammers_client::SignInError::InvalidPassword => Err(ProcessError::InvalidPassword),
                grammers_client::SignInError::Other(_) => Err(ProcessError::OtherSignInError),
            },
        }?;

        let path = format!("sessions/{}.session", user.id());
        fs::write(&path, "").map_err(|_| ProcessError::SessionFileIsNotExist)?;
        tg_client
            .session()
            .save_to_file(path)
            .map_err(|_| ProcessError::CannotSaveSessionInFile)?;

        if Self::find_chat_with_files(tg_client.iter_dialogs(), &user)
            .await
            .is_err()
        {
            tg_client
                .invoke(&CreateChat {
                    users: vec![enums::InputUser::User(InputUser {
                        user_id: user.id(),
                        access_hash: match user.pack().access_hash {
                            Some(v) => v,
                            None => return Err(ProcessError::AccessHashIsNone),
                        },
                    })],
                    title: format!("TuchaCloud-{}", user.id()),
                    ttl_period: None,
                })
                .await
                .map_err(|_| ProcessError::CloudGroupIsNotCreated)?;
        }

        let chat = Self::find_chat_with_files(tg_client.iter_dialogs(), &user).await?;

        Ok(ProcessResult::LoggedInWithCode(
            Client::new(tg_client, Some(chat)),
            match user.username() {
                Some(v) => v,
                None => return Err(ProcessError::UsernameIsNone),
            }
            .to_string(),
        ))
    }

    pub async fn upload_files(
        self: Self,
        transferred_files: Vec<PathBuf>,
    ) -> Result<ProcessResult, ProcessError> {
        let tg_client = self.tg_client;
        let tg_chat = self.tg_chat;
        let creator = tg_client
            .get_me()
            .await
            .map_err(|_| ProcessError::CannotGetUserData)?;

        for file in transferred_files {
            let file_metadata = FileMetadata::new(String::from(""));

            let message = InputMessage::text(
                serde_json::to_string(&file_metadata)
                    .map_err(|_| ProcessError::CannotSerializeToString)?,
            )
            .document(
                tg_client
                    .upload_file(file.as_os_str())
                    .await
                    .map_err(|_| ProcessError::CanntoUploadFile)?,
            );

            let _message = &tg_client
                .send_message(
                    &match &tg_chat.clone() {
                        Some(v) => v.clone(),
                        None => {
                            let chat =
                                Self::find_chat_with_files(tg_client.iter_dialogs(), &creator)
                                    .await?;
                            chat
                        }
                    },
                    message,
                )
                .await
                .map_err(|_| ProcessError::MediaMessageIsNotSended)?;
        }

        Ok(ProcessResult::FilesUploaded)
    }

    pub async fn get_uploaded_files(self) -> Result<ProcessResult, ProcessError> {
        let tg_client = self.tg_client;
        let user = tg_client
            .get_me()
            .await
            .map_err(|_| ProcessError::CannotGetUserData)?;

        let tg_chat = match self.tg_chat {
            Some(v) => v.clone(),
            None => {
                let chat = Self::find_chat_with_files(tg_client.iter_dialogs(), &user).await?;
                chat
            }
        };

        let mut files = Vec::<File>::new();
        let mut messages = tg_client.iter_messages(&tg_chat.clone());

        while let Some(message) = messages
            .next()
            .await
            .map_err(|_| ProcessError::CannotReadMessages)?
        {
            if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                if let Some(media) = message.media() {
                    match media {
                        Media::Document(document) => {
                            let file =
                                File::new(file_metadata, message.id(), document.name().to_string());

                            files.push(file);
                        }
                        Media::Sticker(sticker) => {
                            let file = File::new(
                                file_metadata,
                                message.id(),
                                sticker.document.name().to_string(),
                            );

                            files.push(file);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(ProcessResult::UploadedFilesReceived(
            match user.username() {
                Some(v) => v,
                None => return Err(ProcessError::UsernameIsNone),
            }
            .to_string(),
            files,
        ))
    }

    pub async fn download_files(
        self,
        message_ids: Vec<i32>,
    ) -> Result<ProcessResult, ProcessError> {
        let tg_client = self.tg_client;
        let user = tg_client
            .get_me()
            .await
            .map_err(|_| ProcessError::CannotGetUserData)?;
        let chat = Self::find_chat_with_files(tg_client.iter_dialogs(), &user).await?;

        let mut messages = tg_client
            .get_messages_by_id(&chat, &message_ids)
            .await
            .map_err(|_| ProcessError::MessagesNotFound)?
            .into_iter();

        while let Some(Some(message)) = messages.next() {
            if let Some(media) = message.clone().media() {
                async fn download_file(message: Message, name: &str) -> Result<(), ProcessError> {
                    message
                        .download_media(format!(
                            "{}/Downloads/{}",
                            match home_dir() {
                                Some(v) => v,
                                None => return Err(ProcessError::HomeDirectoryIsNone),
                            }
                            .display()
                            .to_string(),
                            name
                        ))
                        .await
                        .map_err(|_| ProcessError::CannotDownloadMedia)?;
                    Ok(())
                }
                match media {
                    Media::Document(document) => download_file(message, document.name()).await?,
                    Media::Sticker(sticker) => {
                        download_file(message, sticker.document.name()).await?
                    }
                    _ => {}
                }
            } else {
                return Err(ProcessError::MessageNotContainsMedia);
            }
        }

        Ok(ProcessResult::FilesDownloaded)
    }
}
