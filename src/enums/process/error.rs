use std::fmt::Display;

use super::result::ProcessResult;

#[derive(Debug, Clone)]
pub enum ProcessError {
    AccessHashIsNone,
    CannotDeleteFile,
    CannotDownloadMedia,
    CannotGetDialogs,
    CannotGetUserData,
    CannotLoadSessionFile,
    CannotParseAPIIDToInteger,
    CannotReadMessages,
    CannotReadSessionsDirectory,
    CannotSerializeToString,
    CannotUploadFile,
    ChatIsNone,
    ClientIsNotConnected,
    CloudGroupIsNotCreated,
    CurrentClientIsNone,
    SessionFileIsNotExist,
    SignUpRequired,
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
    PasswordRequired,
    CannotSaveAPIKeysInFile,
    CannotSaveSessionInFile,
    UserIsNone,
    UsernameIsNone,
}

impl std::error::Error for ProcessError {}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::AccessHashIsNone => write!(f, "access_hash is None."),
            ProcessError::CannotDeleteFile => write!(f, "Cannot delete file."),
            ProcessError::CannotDownloadMedia => write!(f, "Cannot download media from message."),
            ProcessError::CannotGetDialogs => write!(f, "Cannot get dialogs."),
            ProcessError::CannotGetUserData => write!(f, "Cannot get user data."),
            ProcessError::CannotLoadSessionFile => write!(f, "Cannot load session file."),
            ProcessError::CannotParseAPIIDToInteger => write!(f, "Parse api_id to integer"),
            ProcessError::CannotReadMessages => write!(f, "Cannot read messages."),
            ProcessError::CannotReadSessionsDirectory => {
                write!(f, "Cannot read directory with sessions files.")
            }
            ProcessError::CannotSerializeToString => {
                write!(f, "Cannot serialize file metadata to string.")
            }
            ProcessError::CannotUploadFile => write!(f, "Cannot upload file."),
            ProcessError::ChatIsNone => write!(f, "Chat is None."),
            ProcessError::ClientIsNotConnected => write!(f, "Client is not connected."),
            ProcessError::CloudGroupIsNotCreated => write!(f, "Cloud gropup is not created."),
            ProcessError::CurrentClientIsNone => write!(f, "Current client is None."),
            ProcessError::SessionFileIsNotExist => write!(f, "Session file is not exist."),
            ProcessError::SignUpRequired => write!(f, "Sign up required."),
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
            ProcessError::PasswordRequired => write!(f, "Password required."),
            ProcessError::CannotSaveAPIKeysInFile => write!(f, "Cannot save api keys in file"),
            ProcessError::CannotSaveSessionInFile => write!(f, "Cannot save session in file."),
            ProcessError::UserIsNone => write!(f, "User is None."),
            ProcessError::UsernameIsNone => write!(f, "Username is None."),
        }
    }
}

impl ProcessError {
    pub fn to_process_result(self) -> ProcessResult {
        ProcessResult::Error(self)
    }
}
