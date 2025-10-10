use crate::error::ServerError;
use rusqlite::{Row, Transaction};
use sea_query::{Expr, IdenStatic, Query, SqliteQueryBuilder, enum_def};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Config {
    pub id: i64,
    pub text: String,
}

impl TryFrom<&Row<'_>> for Config {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(ConfigIden::Id.as_str())?,
            text: value.get(ConfigIden::Text.as_str())?,
        })
    }
}

impl Config {
    pub fn get(id: i64, transaction: &Transaction) -> Result<Option<String>, ServerError> {
        let (query, values) = Query::select()
            .columns([ConfigIden::Text])
            .from(ConfigIden::Table)
            .and_where(Expr::col(ConfigIden::Id).eq(id))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let mut rows = statement.query(&*values.as_params())?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set(id: i64, text: String, transaction: &Transaction) -> Result<(), ServerError> {
        // First try to update
        let (update_query, update_values) = Query::update()
            .table(ConfigIden::Table)
            .values([(ConfigIden::Text, text.clone().into())])
            .and_where(Expr::col(ConfigIden::Id).eq(id))
            .build_rusqlite(SqliteQueryBuilder);

        let affected = transaction.execute(&update_query, &*update_values.as_params())?;

        if affected == 0 {
            // Insert
            let (insert_query, insert_values) = Query::insert()
                .into_table(ConfigIden::Table)
                .columns([ConfigIden::Id, ConfigIden::Text])
                .values([id.into(), text.into()])?
                .build_rusqlite(SqliteQueryBuilder);

            transaction.execute(&insert_query, &*insert_values.as_params())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database;
    use rusqlite::Connection;

    fn setup_conn() -> Result<Connection, ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        database::migration::run_migration(&tran)?;
        tran.commit()?;
        Ok(conn)
    }

    #[test]
    fn insert_and_find() -> Result<(), ServerError> {
        let mut conn = setup_conn()?;
        let id = 1;
        let text = "Test text".to_string();

        let tran = conn.transaction()?;
        Config::set(id, text.clone(), &tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let fetched = Config::get(id, &tran)?.expect("config text");
        assert_eq!(text, fetched);
        tran.finish()?;

        Ok(())
    }

    #[test]
    fn by_id_not_found() -> Result<(), ServerError> {
        let mut conn = setup_conn()?;
        let tran = conn.transaction()?;
        let result = Config::get(999, &tran)?;
        assert!(result.is_none());
        tran.finish()?;
        Ok(())
    }
}