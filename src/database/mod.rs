pub(crate) mod image;
mod migration;
pub(crate) mod penalty;
pub(crate) mod user;
pub(crate) mod video;
pub(crate) mod wheel;

use crate::error::ServerError;
use rusqlite::Connection;
use std::fs;
use tempfile::NamedTempFile;

pub(crate) fn get_connection() -> Result<Connection, rusqlite::Error> {
    const DATABASE: &str = "data/sqlite.db";
    Connection::open(DATABASE)
}

pub fn init() -> Result<(), ServerError> {
    fs::create_dir_all("data/")?;

    let mut conn = get_connection()?;
    let tran = conn.transaction()?;
    migration::run_migration(&tran)?;
    tran.commit()?;

    Ok(())
}

pub fn backup_database() -> Result<Vec<u8>, ServerError> {
    let temp_file = NamedTempFile::new()?;
    let mut dst = Connection::open(temp_file.path())?;
    let src = get_connection()?;
    rusqlite::backup::Backup::new(&src, &mut dst)?.run_to_completion(
        5,
        std::time::Duration::from_millis(250),
        None,
    )?;
    fs::read(temp_file.path()).map_err(ServerError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() -> Result<(), ServerError> {
        let mut connection = Connection::open_in_memory()?;
        let transaction = connection.transaction()?;
        migration::run_migration(&transaction)?;
        transaction.commit()?;

        Ok(())
    }
}
