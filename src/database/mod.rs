mod migration;
pub mod wheel;

use rusqlite::Connection;
use crate::error::ServerError;
use std::fs;

pub fn get_connection() -> Result<Connection, rusqlite::Error> {
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
