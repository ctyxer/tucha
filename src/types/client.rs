use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc, vec::IntoIter};

use grammers_client::{
    types::{Chat, LoginToken, Media, Message, User},
    Client as TGClient, Config, InputMessage,
};
use grammers_session::Session;
use grammers_tl_types as tl;
use tl::{enums, functions::messages::CreateChat, types::InputUser};

use crate::{
    enums::{ProcessError, ProcessResult},
    utils::{self},
};

use super::{APIKeys, File, FileMetadata};

#[derive(Clone, Debug)]
pub struct Client {
    pub tg_client: TGClient,
    pub chat: Option<Chat>,
    pub user: Option<User>,
}

impl Client {
    pub async fn new(tg_client: TGClient, is_completed: bool) -> Result<Self, ProcessError> {
        if is_completed {
            let user = tg_client
                .get_me()
                .await
                .map_err(|_| ProcessError::CannotGetUserData)?;

            let mut iter_dialogs = tg_client.iter_dialogs();

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
                    return Ok(Self {
                        tg_client,
                        chat: Some(dialog.chat().clone()),
                        user: Some(user),
                    });
                }
            }
            Err(ProcessError::ChatIsNone)
        } else {
            Ok(Self {
                tg_client,
                chat: None,
                user: None,
            })
        }
    }

    fn get_user(&self) -> Result<&User, ProcessError> {
        match &self.user {
            Some(v) => Ok(v),
            None => Err(ProcessError::UserIsNone),
        }
    }

    fn get_chat(&self) -> Result<&Chat, ProcessError> {
        match &self.chat {
            Some(v) => Ok(v),
            None => Err(ProcessError::ChatIsNone),
        }
    }

    fn get_username(&self) -> Result<String, ProcessError> {
        match self.get_user() {
            Ok(user) => match user.username() {
                Some(v) => Ok(v.to_string()),
                None => Err(ProcessError::UsernameIsNone),
            },
            Err(err) => Err(err),
        }
    }

    async fn get_messages_by_id(
        &self,
        message_ids: &[i32],
    ) -> Result<IntoIter<Option<Message>>, ProcessError> {
        Ok(self
            .tg_client
            .get_messages_by_id(self.get_chat()?, &message_ids)
            .await
            .map_err(|_| ProcessError::MessagesNotFound)?
            .into_iter())
    }

    pub async fn connect_to_saved_sessions() -> Result<ProcessResult, ProcessError> {
        let mut session_files =
            fs::read_dir("./sessions").map_err(|_| ProcessError::CannotReadSessionsDirectory)?;
        let mut clients: BTreeMap<String, Self> = BTreeMap::new();

        while let Some(Ok(session_file)) = session_files.next() {
            let path = session_file.path();
            let secret_data = APIKeys::new();

            let tg_client = TGClient::connect(Config {
                session: Session::load_file(path)
                    .map_err(|_| ProcessError::CannotLoadSessionFile)?,
                api_id: secret_data.api_id,
                api_hash: secret_data.api_hash,
                params: Default::default(),
            })
            .await
            .map_err(|_| ProcessError::ClientIsNotConnected)?;

            let client = Self::new(tg_client, true).await?;

            clients.insert(client.get_username()?.to_string(), client);
        }
        Ok(ProcessResult::ConnectedToSavedClients(clients))
    }

    pub async fn send_login_code(phone_number: String) -> Result<ProcessResult, ProcessError> {
        let secret_data = APIKeys::new();

        let tg_client = TGClient::connect(Config {
            session: Session::new(),
            api_id: secret_data.api_id,
            api_hash: secret_data.api_hash,
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
            Client::new(tg_client, false).await?,
        ))
    }

    pub async fn sign_in(
        self: Self,
        received_code: String,
        login_token: Arc<LoginToken>,
        user_password: String,
    ) -> Result<ProcessResult, ProcessError> {
        let user = match self.tg_client.sign_in(&login_token, &received_code).await {
            Ok(v) => Ok(v),
            Err(err) => match err {
                grammers_client::SignInError::SignUpRequired {
                    terms_of_service: _,
                } => Err(ProcessError::SignUpRequired),
                grammers_client::SignInError::PasswordRequired(password_token) => {
                    if !user_password.is_empty() {
                        match self
                            .tg_client
                            .check_password(password_token, user_password)
                            .await
                        {
                            Ok(v) => Ok(v),
                            Err(_) => Err(ProcessError::OtherSignInError),
                        }
                    } else {
                        Err(ProcessError::PasswordRequired)
                    }
                }
                grammers_client::SignInError::InvalidCode => Err(ProcessError::InvalidCode),
                grammers_client::SignInError::InvalidPassword => Err(ProcessError::InvalidPassword),
                grammers_client::SignInError::Other(_) => Err(ProcessError::OtherSignInError),
            },
        }?;

        let path = format!("sessions/{}.session", user.id());
        fs::write(&path, "").map_err(|_| ProcessError::SessionFileIsNotExist)?;
        self.tg_client
            .session()
            .save_to_file(path)
            .map_err(|_| ProcessError::CannotSaveSessionInFile)?;

        if self.get_chat().is_err() {
            self.tg_client
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

        let client = Client::new(self.tg_client, true).await?;

        Ok(ProcessResult::LoggedIn(
            client.clone(),
            client.get_username()?,
        ))
    }

    pub async fn upload_files(
        self: Self,
        transferred_files: Vec<PathBuf>,
    ) -> Result<ProcessResult, ProcessError> {
        for file in transferred_files {
            let file_metadata = FileMetadata::new(String::from(""));

            let message = InputMessage::text(
                serde_json::to_string(&file_metadata)
                    .map_err(|_| ProcessError::CannotSerializeToString)?,
            )
            .document(
                self.tg_client
                    .upload_file(file.as_os_str())
                    .await
                    .map_err(|_| ProcessError::CannotUploadFile)?,
            );

            let _message = &self
                .tg_client
                .send_message(self.get_chat()?, message)
                .await
                .map_err(|_| ProcessError::MediaMessageIsNotSended)?;
        }

        Ok(ProcessResult::FilesUploaded)
    }

    pub async fn get_uploaded_files(self) -> Result<ProcessResult, ProcessError> {
        let mut files = Vec::<File>::new();
        let mut messages = self.tg_client.iter_messages(&self.get_chat()?.clone());

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
            self.get_username()?,
            files,
        ))
    }

    pub async fn download_files(
        self,
        message_ids: Vec<i32>,
    ) -> Result<ProcessResult, ProcessError> {
        let mut messages = self.get_messages_by_id(&message_ids).await?;

        while let Some(Some(message)) = messages.next() {
            if let Some(media) = message.clone().media() {
                match media {
                    Media::Document(document) => {
                        utils::download_file(message, document.name()).await?
                    }
                    Media::Sticker(sticker) => {
                        utils::download_file(message, sticker.document.name()).await?
                    }
                    _ => {}
                }
            } else {
                return Err(ProcessError::MessageNotContainsMedia);
            }
        }

        Ok(ProcessResult::FilesDownloaded)
    }
    pub async fn delete_files(self, message_ids: Vec<i32>) -> Result<ProcessResult, ProcessError> {
        let mut messages = self.get_messages_by_id(&message_ids).await?;

        while let Some(Some(message)) = messages.next() {
            message
                .delete()
                .await
                .map_err(|_| ProcessError::CannotDeleteFile)?;
        }

        Ok(ProcessResult::FilesDeleted)
    }
}
