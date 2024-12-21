use crate::error::ServerError;
use log::info;

const VERSION: u32 = 1;

pub fn run_migration(transaction: &rusqlite::Transaction) -> Result<(), ServerError> {
    let mut version =
        transaction.query_row("PRAGMA user_version;", (), |row| {
            row.get::<_, u32>(0)
        })?;

    if version == VERSION {
        info!("version is up to date, skip database migration.");
        return Ok(());
    }

    macro_rules! migrate {
        ($id:tt, $file:tt) => {
            if version + 1 == $id {
                info!(
                    "migrate database from version {} to version {}",
                    version,
                    version + 1
                );
                transaction.execute_batch(include_str!($file))?;
                version += 1;
            }
        };
    }

    migrate!(1, "001_wheel_tables.sql");

    if version != VERSION {
        Err(ServerError::Internal(format!(
            "fail to migrate database from version {} to version {}",
            version, VERSION
        )))
    } else {
        // PRAGMA does not support value binding
        transaction.execute(
            format!("PRAGMA user_version = {};", version).as_str(),
            (),
        )?;
        Ok(())
    }
}
