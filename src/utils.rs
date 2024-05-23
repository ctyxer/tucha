use std::sync::mpsc::Sender;

use dirs::home_dir;
use grammers_client::types::Message;

use crate::enums::{ProcessError, ProcessResult};

pub fn send_result(
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

pub fn get_home_directory() -> Result<String, ProcessError> {
    match home_dir() {
        Some(v) => Ok(v.display().to_string()),
        None => return Err(ProcessError::HomeDirectoryIsNone),
    }
}

pub async fn download_file(message: Message, name: &str) -> Result<(), ProcessError> {
    message
        .download_media(format!("{}/Downloads/{}", get_home_directory()?, name))
        .await
        .map_err(|_| ProcessError::CannotDownloadMedia)?;
    Ok(())
}
