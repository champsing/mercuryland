use crate::error::ServerError;

const VERSION: u32 = 12;

pub fn run_migration(transaction: &rusqlite::Transaction) -> Result<(), ServerError> {
    let mut version =
        transaction.query_row("PRAGMA user_version;", (), |row| row.get::<_, u32>(0))?;

    if version == VERSION {
        log::info!("version is up to date, skip database migration.");
        return Ok(());
    }

    macro_rules! migrate {
        ($id:tt, $file:tt) => {
            if version + 1 == $id {
                log::info!(
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
    migrate!(2, "002_user_tables.sql");
    migrate!(3, "003_rename_user.sql");
    migrate!(4, "004_coin_add_col.sql");
    migrate!(5, "005_coin_add_col_discord_id.sql");
    migrate!(6, "006_video_tables.sql");
    migrate!(7, "007_user_table_restructure.sql");
    migrate!(8, "008_image_tables.sql");
    migrate!(9, "009_penalty_tables.sql");
    migrate!(10, "010_config_tables.sql");
    migrate!(11, "011_anonymous_table.sql");
    migrate!(12, "012_drop_anonymous_content.sql");

    if version != VERSION {
        Err(format!(
            "fail to migrate database from version {} to version {}",
            version, VERSION
        )
        .into())
    } else {
        // PRAGMA does not support value binding
        transaction.execute(format!("PRAGMA user_version = {};", version).as_str(), ())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn user_version(conn: &Connection) -> Result<u32, ServerError> {
        let version = conn.query_row("PRAGMA user_version;", (), |row| row.get(0))?;
        Ok(version)
    }

    #[test]
    fn migrates_from_scratch_to_latest() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        run_migration(&tran)?;
        tran.commit()?;

        assert_eq!(user_version(&conn)?, VERSION);
        Ok(())
    }

    #[test]
    fn skips_when_already_up_to_date() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        conn.pragma_update(None, "user_version", &VERSION)?;

        let tran = conn.transaction()?;
        run_migration(&tran)?;
        tran.commit()?;

        assert_eq!(user_version(&conn)?, VERSION);
        Ok(())
    }

    #[test]
    fn errors_on_future_version() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        conn.pragma_update(None, "user_version", &(VERSION + 1))?;

        let tran = conn.transaction()?;
        assert!(run_migration(&tran).is_err());
        Ok(())
    }
}
