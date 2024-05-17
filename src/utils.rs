use std::env;

use crate::handlers::process::error::ProcessError;

pub fn get_secret_data() -> Result<(i32, String), ProcessError> {
    Ok((
        env::var("API_HASH")
            .map_err(|_| ProcessError::InvalidAPI)?
            .parse::<i32>()
            .map_err(|_| ProcessError::InvalidAPI)?,
        env::var("API_HASH").map_err(|_| ProcessError::InvalidAPI)?,
    ))
}
