use chrono::{DateTime, Utc};
use rand::distributions::{Alphanumeric, DistString};
use rusqlite::{Row, Transaction};
use sea_query::{enum_def, Alias, Expr, ExprTrait, Func, IdenStatic, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

use crate::error::ServerError;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Wheel {
    #[serde(default)]
    pub id: u16,
    pub secret: String,
    pub content: JsonValue,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for Wheel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Wheel {}

impl TryFrom<&Row<'_>> for Wheel {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.get(WheelIden::Id.as_str())?,
            secret: value.get(WheelIden::Secret.as_str())?,
            content: value.get(WheelIden::Content.as_str())?,
            updated_at: value.get(WheelIden::UpdatedAt.as_str())?,
        })
    }
}

impl Wheel {
    fn id_gen(transaction: &Transaction) -> Result<u16, ServerError> {
        let i = Alias::new("i");
        let t = Alias::new("t");

        let (query, values) = Query::select()
            .column(i.clone())
            .from_subquery(
                Query::select()
                    .expr_as(Func::abs(Expr::cust("RANDOM()")).modulo(65536), i.clone())
                    .take(),
                t,
            )
            .and_where(
                Expr::col(i).not_in_subquery(
                    Query::select()
                        .column(WheelIden::Id)
                        .from(WheelIden::Table)
                        .take(),
                ),
            )
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        loop {
            if let Some(new_id) = statement
                .query_and_then(&*values.as_params(), |row| row.get::<_, u16>(0))?
                .next()
            {
                let id = new_id?;
                return Ok(id);
            }
        }
    }

    pub fn by_id(id: u16, transaction: &Transaction) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                WheelIden::Id,
                WheelIden::Secret,
                WheelIden::Content,
                WheelIden::UpdatedAt,
            ])
            .from(WheelIden::Table)
            .and_where(Expr::col(WheelIden::Id).eq(id))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| Wheel::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }

    pub fn create(now: DateTime<Utc>, transaction: &Transaction) -> Result<Self, ServerError> {
        let value = Self {
            id: Self::id_gen(transaction)?,
            secret: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            content: json!([]),
            updated_at: now,
        };

        let (query, values) = Query::insert()
            .into_table(WheelIden::Table)
            .columns([
                WheelIden::Id,
                WheelIden::Secret,
                WheelIden::Content,
                WheelIden::UpdatedAt,
            ])
            .values([
                value.id.into(),
                value.secret.clone().into(),
                value.content.clone().into(),
                value.updated_at.into(),
            ])?
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(value)
    }

    pub fn update(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::update()
            .table(WheelIden::Table)
            .values([
                (WheelIden::Content, self.content.clone().into()),
                (WheelIden::UpdatedAt, self.updated_at.into()),
            ])
            .and_where(Expr::col(WheelIden::Id).eq(self.id))
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(())
    }

    pub fn purge(before: DateTime<Utc>, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::delete()
            .from_table(WheelIden::Table)
            .and_where(Expr::col(WheelIden::UpdatedAt).lt(before))
            .build_rusqlite(SqliteQueryBuilder);
        transaction.execute(&query, &*values.as_params())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::database;
    use chrono::TimeDelta;
    use rusqlite::Connection;

    #[test]
    fn id_gen() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        database::migration::run_migration(&tran)?;

        let _ = Wheel::id_gen(&tran)?;

        Ok(())
    }

    #[test]
    fn create() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        database::migration::run_migration(&tran)?;
        tran.commit()?;

        let mut records = HashSet::new();
        let now = Utc::now();
        for _ in 0..4096 {
            let tran = conn.transaction()?;
            let w = Wheel::create(now, &tran)?;
            tran.commit()?;
            records.insert(w.id);
        }

        assert_eq!(records.len(), 4096);

        Ok(())
    }

    #[test]
    fn update() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        database::migration::run_migration(&tran)?;
        tran.commit()?;

        let t0 = Utc::now();
        let tran = conn.transaction()?;
        let mut w = Wheel::create(t0, &tran)?;
        assert_eq!(t0, w.updated_at);
        tran.commit()?;

        let tran = conn.transaction()?;
        let w0 = Wheel::by_id(w.id, &tran)?.expect("no value");
        assert_eq!(w.id, w0.id);
        assert_eq!(w.secret, w0.secret);
        assert_eq!(w.content, w0.content);
        assert_eq!(t0, w0.updated_at);
        tran.finish()?;

        let t1 = Utc::now();
        let tran = conn.transaction()?;
        w.updated_at = t1;
        w.secret = String::from("some secret");
        w.content = json!(["some", "random", "content"]);
        w.update(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let w1 = Wheel::by_id(w.id, &tran)?.expect("no value");
        assert_eq!(w.id, w1.id);
        assert_eq!(w0.secret, w1.secret); // secret should not change.
        assert_eq!(w.content, w1.content);
        assert_eq!(t1, w1.updated_at);
        tran.finish()?;

        Ok(())
    }

    #[test]
    fn purge() -> Result<(), ServerError> {
        let mut conn = Connection::open_in_memory()?;
        let tran = conn.transaction()?;
        database::migration::run_migration(&tran)?;
        tran.commit()?;

        let now = Utc::now();
        let t0 = now - TimeDelta::days(2);
        let t1 = now - TimeDelta::days(1);
        let t2 = now;

        let tran = conn.transaction()?;
        let w0 = Wheel::create(t0, &tran)?;
        let w1 = Wheel::create(t2, &tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        Wheel::purge(t1, &tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        assert!(Wheel::by_id(w0.id, &tran)?.is_none());
        assert!(Wheel::by_id(w1.id, &tran)?.is_some());
        tran.finish()?;

        Ok(())
    }
}
