use crate::error::ServerError;
use chrono::{DateTime, Utc};
use rusqlite::{Row, Transaction};
use sea_query::{enum_def, Expr, IdenStatic, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct User {
    pub id: String,
    pub coin: i64,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl TryFrom<&Row<'_>> for User {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(UserIden::Id.as_str())?,
            coin: value.get(UserIden::Coin.as_str())?,
            updated_at: value.get(UserIden::UpdatedAt.as_str())?,
        })
    }
}

impl User {
    pub fn insert(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::insert()
            .into_table(UserIden::Table)
            .columns([UserIden::Id, UserIden::Coin, UserIden::UpdatedAt])
            .values([
                self.id.clone().into(),
                self.coin.into(),
                self.updated_at.into(),
            ])?
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(())
    }

    pub fn by_id(
        id: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([UserIden::Id, UserIden::Coin, UserIden::UpdatedAt])
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::Id).eq(id.into()))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| User::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }

    pub fn update(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::update()
            .table(UserIden::Table)
            .values([(UserIden::Coin, self.coin.into())])
            .and_where(Expr::col(UserIden::Id).eq(self.id.clone()))
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database;
    use rusqlite::Connection;

    #[test]
    fn insert() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        database::migration::run_migration(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let user = User {
            id: String::from("test_id"),
            coin: 0,
            updated_at: Utc::now(),
        };
        user.insert(&tran)?;
        tran.commit()?;

        Ok(())
    }

    #[test]
    fn update() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        database::migration::run_migration(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let mut u = User {
            id: String::from("test_id"),
            coin: 0,
            updated_at: Utc::now(),
        };
        u.insert(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let u0 = User::by_id(u.id.clone(), &tran)?.expect("no value");
        assert_eq!(u.coin, u0.coin);
        tran.finish()?;

        let tran = conn.transaction()?;
        u.coin = 10;
        u.update(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let u1 = User::by_id(u.id.clone(), &tran)?.expect("no value");
        assert_eq!(u.coin, u1.coin);
        tran.finish()?;

        Ok(())
    }
}
