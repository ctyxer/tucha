use std::fmt::Display;

use super::error::ProcessError;

pub enum CurrentProcess{
    Idle, 
    Error(ProcessError),
    ConnectingToAllSavedClients,
    GettingUploadedFiles,
    SendingLoginCode,
    LogInWithCode,
    UploadingFiles,
    DownloadingFiles,
    DeletingFiles,
}

impl Display for CurrentProcess{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentProcess::Idle => write!(f, "tucha"),
            CurrentProcess::Error(process_error) => write!(f, "{}", process_error),
            CurrentProcess::ConnectingToAllSavedClients => write!(f, "Connecting to all saved clients..."),
            CurrentProcess::SendingLoginCode => write!(f, "Sending login code..."),
            CurrentProcess::LogInWithCode => write!(f, "Log in..."),
            CurrentProcess::UploadingFiles => write!(f, "Uploading files..."),
            CurrentProcess::GettingUploadedFiles => write!(f, "Getting uploaded files..."),
            CurrentProcess::DownloadingFiles => write!(f, "Downloading files..."),
            CurrentProcess::DeletingFiles => write!(f, "Deleting files..."),
        }
    }
}
