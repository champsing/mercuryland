use crate::error::ServerError;
use chrono::{DateTime, Utc};
use rusqlite::{Row, Transaction};
use sea_query::{enum_def, Expr, IdenStatic, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Coin {
    pub id: String,
    pub discord_id: u64,
    pub coin: i64,
    pub display: String,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for Coin {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Coin {}

impl TryFrom<&Row<'_>> for Coin {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(CoinIden::Id.as_str())?,
            discord_id: value.get(CoinIden::DiscordId.as_str())?,
            coin: value.get(CoinIden::Coin.as_str())?,
            display: value.get(CoinIden::Display.as_str())?,
            updated_at: value.get(CoinIden::UpdatedAt.as_str())?,
        })
    }
}

impl Coin {
    pub fn insert(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::insert()
            .into_table(CoinIden::Table)
            .columns([
                CoinIden::Id,
                CoinIden::DiscordId,
                CoinIden::Coin,
                CoinIden::Display,
                CoinIden::UpdatedAt,
            ])
            .values([
                self.id.clone().into(),
                self.discord_id.clone().into(),
                self.coin.into(),
                self.display.clone().into(),
                self.updated_at.into(),
            ])?
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(())
    }

    pub fn by_youtube(
        id: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                CoinIden::Id,
                CoinIden::DiscordId,
                CoinIden::Coin,
                CoinIden::Display,
                CoinIden::UpdatedAt,
            ])
            .from(CoinIden::Table)
            .and_where(Expr::col(CoinIden::Id).eq(id.into()))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| Coin::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }

    pub fn by_discord(
        id: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                CoinIden::Id,
                CoinIden::DiscordId,
                CoinIden::Coin,
                CoinIden::Display,
                CoinIden::UpdatedAt,
            ])
            .from(CoinIden::Table)
            .and_where(Expr::col(CoinIden::DiscordId).eq(id.into()))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| Coin::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }

    pub fn get_or_create(
        id: impl Into<String>,
        display: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<Self, ServerError> {
        let id = id.into();
        if let Some(user) = Self::by_youtube(id.clone(), transaction)? {
            Ok(user)
        } else if let Some(user) = Self::by_discord(id.clone(), transaction)? {
            Ok(user)
        } else {
            let user = Self {
                id,
                discord_id: 0,
                coin: 0,
                display: display.into(),
                updated_at: Utc::now(),
            };
            user.insert(transaction)?;
            Ok(user)
        }
    }

    pub fn update(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::update()
            .table(CoinIden::Table)
            .values([
                (CoinIden::Coin, self.coin.into()),
                (CoinIden::DiscordId, self.discord_id.into()),
                (CoinIden::Display, self.display.clone().into()),
                (CoinIden::UpdatedAt, self.updated_at.into()),
            ])
            .and_where(Expr::col(CoinIden::Id).eq(self.id.clone()))
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
        let user = Coin {
            id: String::from("test_id"),
            discord_id: 0,
            coin: 0,
            display: String::from("test_user_1"),
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
        let mut u = Coin {
            id: String::from("test_id"),
            discord_id: 0,
            coin: 0,
            display: String::from("test_user_1"),
            updated_at: Utc::now(),
        };
        u.insert(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let u0 = Coin::by_youtube(u.id.clone(), &tran)?.expect("no value");
        let u1 = Coin::by_discord(u.discord_id.clone().to_string(), &tran)?.expect("no value");
        assert_eq!(u.coin, u0.coin);
        assert_eq!(u.display, u0.display);
        assert_eq!(u.updated_at, u0.updated_at);
        assert_eq!(u.coin, u1.coin);
        assert_eq!(u.display, u1.display);
        assert_eq!(u.updated_at, u1.updated_at);
        tran.finish()?;

        let tran = conn.transaction()?;
        u.coin = 10;
        u.display = String::from("test_user_2");
        u.updated_at = Utc::now();
        u.update(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let u1 = Coin::by_youtube(u.id.clone(), &tran)?.expect("no value");
        assert_eq!(u.coin, u1.coin);
        assert_eq!(u.display, u1.display);
        assert_eq!(u.updated_at, u1.updated_at);
        tran.finish()?;

        Ok(())
    }
}
