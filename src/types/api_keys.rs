use std::{
    fs::{self, File},
    io::BufReader,
};

use serde::{Deserialize, Serialize};

use crate::enums::process::error::ProcessError;

#[derive(Serialize, Deserialize)]
pub struct APIKeys {
    pub api_id: i32,
    pub api_hash: String,
}

impl APIKeys {
    pub fn new(api_id: i32, api_hash: String) -> Self {
        Self{ 
            api_hash, 
            api_id
        }
    }

    pub fn get() -> Result<Self, ProcessError> {
        let file = File::open("telegram_app_api.json").map_err(|_| ProcessError::InvalidAPI)?;
        let buf_reader = BufReader::new(file);
        Ok(serde_json::from_reader(buf_reader).map_err(|_| ProcessError::InvalidAPI)?)
    }

    pub fn save(self) -> Result<(), ProcessError> {
        fs::write(
            "telegram_app_api.json",
            serde_json::to_string(&self).map_err(|_| ProcessError::CannotSerializeToString)?,
        )
        .map_err(|_| ProcessError::CannotSaveAPIKeysInFile)?;

        Ok(())
    }
}
