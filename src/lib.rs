mod error;
mod config;
mod database;
pub mod webpage;
pub mod discord;

use error::ServerError;

pub fn init() -> Result<(), ServerError> {
    database::init()?;
    Ok(())
}
