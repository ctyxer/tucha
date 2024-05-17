use std::fmt::Display;

pub enum CurrentProcess{
    Idle, 
    ConnectingToAllSavedClients,
    GettingUploadedFiles,
    SendingLoginCode,
    LogInWithCode,
    UploadingFiles,
}

impl Display for CurrentProcess{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentProcess::Idle => write!(f, "Tucha"),
            CurrentProcess::ConnectingToAllSavedClients => write!(f, "Connecting to all saved clients..."),
            CurrentProcess::SendingLoginCode => write!(f, "Sending login code..."),
            CurrentProcess::LogInWithCode => write!(f, "Log in..."),
            CurrentProcess::UploadingFiles => write!(f, "Uploading files..."),
            CurrentProcess::GettingUploadedFiles => write!(f, "Getting uploaded files..."),
        }
    }
}
