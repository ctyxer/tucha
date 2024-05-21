use std::fmt::Display;

use grammers_client::types::PasswordToken;

#[derive(Debug, Clone)]
pub enum ProcessError {
    AccessHashIsNone,
    CannotDownloadMedia,
    CannotGetDialogs,
    CannotGetUserData,
    CannotLoadSessionFile,
    CannotReadMessages,
    CannotReadSessionsDirectory,
    CannotSerializeToString,
    CanntoUploadFile,
    ChatIsNotFound,
    ClientIsNotConnected,
    CloudGroupIsNotCreated,
    CurrentClientIsNone,
    SessionFileIsNotExist,
    HomeDirectoryIsNone,
    IncompleteClientIsNone,
    InvalidAPI,
    InvalidCode,
    InvalidPassword,
    LoginCodeIsNotSended,
    LoginTokenIsNone,
    MediaMessageIsNotSended,
    MessageNotContainsMedia,
    MessagesNotFound,
    OtherSignInError,
    PasswordRequired(PasswordToken),
    CannotSaveSessionInFile,
    UsernameIsNone,
}

impl std::error::Error for ProcessError {}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::AccessHashIsNone => write!(f, "access_hash is None."),
            ProcessError::CannotDownloadMedia => write!(f, "Cannot download media from message."),
            ProcessError::CannotGetDialogs => write!(f, "Cannot get dialogs."),
            ProcessError::CannotGetUserData => write!(f, "Cannot get user data."),
            ProcessError::CannotLoadSessionFile => write!(f, "Cannot load session file."),
            ProcessError::CannotReadMessages => write!(f, "Cannot read messages."),
            ProcessError::CannotReadSessionsDirectory => {
                write!(f, "Cannot read directory with sessions files.")
            }
            ProcessError::CannotSerializeToString => {
                write!(f, "Cannot serialize file metadata to string.")
            }
            ProcessError::CanntoUploadFile => write!(f, "Cannot upload file."),
            ProcessError::ChatIsNotFound => write!(f, "Chat is not found."),
            ProcessError::ClientIsNotConnected => write!(f, "Client is not connected."),
            ProcessError::CloudGroupIsNotCreated => write!(f, "Cloud gropup is not created."),
            ProcessError::CurrentClientIsNone => write!(f, "Current client is None."),
            ProcessError::SessionFileIsNotExist => write!(f, "Session file is not exist."),
            ProcessError::HomeDirectoryIsNone => write!(f, "Home directory is None."),
            ProcessError::IncompleteClientIsNone => {
                write!(f, "Incomplete telegram client is None.")
            }
            ProcessError::InvalidAPI => write!(f, "Invalid API."),
            ProcessError::InvalidCode => write!(f, "Invalid code."),
            ProcessError::InvalidPassword => write!(f, "Invalid password."),
            ProcessError::LoginCodeIsNotSended => write!(f, "Login code is not sended."),
            ProcessError::LoginTokenIsNone => write!(f, "Login token is None."),
            ProcessError::MediaMessageIsNotSended => write!(f, "Media message is not sended."),
            ProcessError::MessageNotContainsMedia => write!(f, "Message not contains media."),
            ProcessError::MessagesNotFound => write!(f, "Message not found."),
            ProcessError::OtherSignInError => write!(f, "Other sign in error."),
            ProcessError::PasswordRequired(_) => write!(f, "Password required."),
            ProcessError::CannotSaveSessionInFile => write!(f, "Cannot save session in file."),
            ProcessError::UsernameIsNone => write!(f, "Username is None."),
        }
    }
}
