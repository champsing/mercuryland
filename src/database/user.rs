use crate::error::ServerError;
use chrono::{DateTime, Utc};
use rusqlite::{Row, Transaction};
use sea_query::{Expr, IdenStatic, Query, SqliteQueryBuilder, enum_def};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct User {
    pub youtube: String,
    pub discord: Option<u64>,
    pub coin: i64,
    pub display: String,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.youtube == other.youtube
    }
}

impl Eq for User {}

impl TryFrom<&Row<'_>> for User {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            youtube: value.get(UserIden::Youtube.as_str())?,
            discord: value.get(UserIden::Discord.as_str())?,
            coin: value.get(UserIden::Coin.as_str())?,
            display: value.get(UserIden::Display.as_str())?,
            updated_at: value.get(UserIden::UpdatedAt.as_str())?,
        })
    }
}

impl User {
    pub fn insert(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::insert()
            .into_table(UserIden::Table)
            .columns([
                UserIden::Youtube,
                UserIden::Discord,
                UserIden::Coin,
                UserIden::Display,
                UserIden::UpdatedAt,
            ])
            .values([
                self.youtube.clone().into(),
                self.discord.map(|d| d as i64).into(),
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
                UserIden::Youtube,
                UserIden::Discord,
                UserIden::Coin,
                UserIden::Display,
                UserIden::UpdatedAt,
            ])
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::Youtube).eq(id.into()))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| User::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }

    pub fn by_discord(
        id: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<Option<Self>, ServerError> {
        let id_str = id.into();
        let discord_id = id_str.parse::<u64>().map_err(|_| ServerError::Internal("Invalid discord id".to_string()))?;
        
        let (query, values) = Query::select()
            .columns([
                UserIden::Youtube,
                UserIden::Discord,
                UserIden::Coin,
                UserIden::Display,
                UserIden::UpdatedAt,
            ])
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::Discord).eq(discord_id))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| User::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }

    pub fn all(transaction: &Transaction) -> Result<Vec<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                UserIden::Youtube,
                UserIden::Discord,
                UserIden::Coin,
                UserIden::Display,
                UserIden::UpdatedAt,
            ])
            .from(UserIden::Table)
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| User::try_from(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(value)
    }
    pub fn update_display(
        id: impl Into<String>,
        display: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<(), ServerError> {
        let id = id.into();
        let mut conditions = vec![Expr::col(UserIden::Youtube).eq(id.clone())];
        
        if let Ok(discord_id) = id.parse::<u64>() {
            conditions.push(Expr::col(UserIden::Discord).eq(discord_id));
        }
        
        let (query, values) = Query::update()
            .table(UserIden::Table)
            .values([(UserIden::Display, display.into().into())])
            .and_where(conditions.into_iter().fold(Expr::cust("1=0"), |acc, expr| acc.or(expr)))
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(())
    }

    pub fn get_or_create(
        id: impl Into<String>,
        display: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<Self, ServerError> {
        let id = id.into();
        let display = display.into();
        Self::update_display(id.clone(), display.clone(), transaction)?;
        if let Some(user) = Self::by_youtube(id.clone(), transaction)? {
            Ok(user)
        } else if let Ok(discord_id) = id.parse::<u64>() {
            if let Some(user) = Self::by_discord(discord_id.to_string(), transaction)? {
                Ok(user)
            } else {
                let user = Self {
                    youtube: id,
                    discord: None,
                    coin: 0,
                    display: display.into(),
                    updated_at: Utc::now(),
                };
                user.insert(transaction)?;
                Ok(user)
            }
        } else {
            let user = Self {
                youtube: id,
                discord: None,
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
            .table(UserIden::Table)
            .values([
                (UserIden::Coin, self.coin.into()),
                (UserIden::Discord, self.discord.map(|d| d as i64).into()),
                (UserIden::Display, self.display.clone().into()),
                (UserIden::UpdatedAt, self.updated_at.into()),
            ])
            .and_where(Expr::col(UserIden::Youtube).eq(self.youtube.clone()))
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
            youtube: String::from("test_id"),
            discord: None,
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
        let mut u = User {
            youtube: String::from("test_id"),
            discord: Some(123456),
            coin: 0,
            display: String::from("test_user_1"),
            updated_at: Utc::now(),
        };
        u.insert(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let u0 = User::by_youtube(u.youtube.clone(), &tran)?.expect("no value");
        let u1 = User::by_discord("123456".to_string(), &tran)?.expect("no value");
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
        let u1 = User::by_youtube(u.youtube.clone(), &tran)?.expect("no value");
        assert_eq!(u.coin, u1.coin);
        assert_eq!(u.display, u1.display);
        assert_eq!(u.updated_at, u1.updated_at);
        tran.finish()?;

        Ok(())
    }
}
