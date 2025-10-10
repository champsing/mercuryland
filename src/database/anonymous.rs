use crate::error::ServerError;
use chrono::{DateTime, Utc};
use rusqlite::{Row, Transaction};
use sea_query::{IdenStatic, Query, SqliteQueryBuilder, enum_def};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Anonymous {
    pub id: i64,
    pub author: i64,
    pub content: String,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<&Row<'_>> for Anonymous {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(AnonymousIden::Id.as_str())?,
            author: value.get(AnonymousIden::Author.as_str())?,
            content: value.get(AnonymousIden::Content.as_str())?,
            updated_at: value.get(AnonymousIden::UpdatedAt.as_str())?,
        })
    }
}

impl Anonymous {
    pub fn insert(&self, transaction: &Transaction) -> Result<i64, ServerError> {
        let (query, values) = Query::insert()
            .into_table(AnonymousIden::Table)
            .columns([
                AnonymousIden::Author,
                AnonymousIden::Content,
                AnonymousIden::UpdatedAt,
            ])
            .values([
                self.author.into(),
                self.content.clone().into(),
                self.updated_at.into(),
            ])?
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(transaction.last_insert_rowid())
    }

    pub fn all(transaction: &Transaction) -> Result<Vec<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                AnonymousIden::Id,
                AnonymousIden::Author,
                AnonymousIden::Content,
                AnonymousIden::UpdatedAt,
            ])
            .from(AnonymousIden::Table)
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| Anonymous::try_from(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn insert_and_find() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        crate::database::migration::run_migration(&tran)?;
        tran.commit()?;

        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        crate::database::migration::run_migration(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let a = Anonymous {
            id: 0, // will be set by insert
            author: 123456789,
            content: String::from("test content"),
            updated_at: Utc::now(),
        };
        let inserted_id = a.insert(&tran)?;
        assert!(inserted_id > 0);
        tran.commit()?;

        let tran = conn.transaction()?;
        let all = Anonymous::all(&tran)?;
        assert_eq!(all.len(), 1);
        let a0 = &all[0];
        assert_eq!(a.author, a0.author);
        assert_eq!(a.content, a0.content);
        assert_eq!(a.updated_at, a0.updated_at);
        tran.finish()?;

        Ok(())
    }
}