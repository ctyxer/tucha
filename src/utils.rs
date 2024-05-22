use std::sync::mpsc::Sender;

use dirs::home_dir;
use grammers_client::types::Message;

use crate::{
    enums::process::{error::ProcessError, result::ProcessResult},
    types::api_keys::APIKeys,
};

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

pub fn store_api(api_id: String, api_hash: String) -> Result<ProcessResult, ProcessError> {
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
