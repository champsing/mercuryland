use crate::error::ServerError;
use chrono::{DateTime, Utc};
use rusqlite::{Row, Transaction};
use sea_query::{enum_def, Expr, IdenStatic, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Verify {
    pub yt_ch_id: String,
    pub verification_code: String,
    pub generated_at: DateTime<Utc>,
}

impl PartialEq for Verify {
    fn eq(&self, other: &Self) -> bool {
        self.verification_code == other.verification_code
    }
}

impl Eq for Verify {}

impl TryFrom<&Row<'_>> for Verify {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            yt_ch_id: value.get(VerifyIden::YtChId.as_str())?,
            verification_code: value.get(VerifyIden::VerificationCode.as_str())?,
            generated_at: value.get(VerifyIden::GeneratedAt.as_str())?,
        })
    }
}

impl Verify {
    pub fn insert(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::insert()
            .into_table(VerifyIden::Table)
            .columns([
                VerifyIden::YtChId,
                VerifyIden::VerificationCode,
                VerifyIden::GeneratedAt,
            ])
            .values([
                self.yt_ch_id.clone().into(),
                self.verification_code.clone().into(),
                self.generated_at.clone().into(),
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
            .columns([
                VerifyIden::YtChId,
                VerifyIden::VerificationCode,
                VerifyIden::GeneratedAt,
            ])
            .from(VerifyIden::Table)
            .and_where(Expr::col(VerifyIden::YtChId).eq(id.into()))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| Verify::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }
    
    pub fn by_code(
        code: impl Into<String>,
        transaction: &Transaction,
    ) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                VerifyIden::YtChId,
                VerifyIden::VerificationCode,
                VerifyIden::GeneratedAt,
            ])
            .from(VerifyIden::Table)
            .and_where(Expr::col(VerifyIden::VerificationCode).eq(code.into()))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let value = statement
            .query_and_then(&*values.as_params(), |row| Verify::try_from(row))?
            .next();

        Ok(value.transpose()?)
    }

    // return the code to frontend
    // pub fn get_or_create(
    //     id: impl Into<String>,
    //     transaction: &Transaction,
    // ) -> Result<Self, ServerError> {
    //     let id = id.into();
    //     if let Some(user) = Self::by_id(id.clone(), transaction)? {
    //         Ok(user)
    //     } else {
    //         let user = Self {
    //             yt_ch_id: id.clone(),
    //             verification_code: String::new(),
    //             generated_at: Utc::now(),
    //         };
    //         user.insert(transaction)?;
    //         Ok(user)
    //     }
    // }

    // call this function to update the verification code every 10 minutes
    pub fn update(&self, transaction: &Transaction) -> Result<(), ServerError> {
        let (query, values) = Query::update()
            .table(VerifyIden::Table)
            .values([
                (VerifyIden::YtChId, self.yt_ch_id.clone().into()),
                (VerifyIden::VerificationCode, self.verification_code.clone().into()),
                (VerifyIden::GeneratedAt, self.generated_at.clone().into()),
            ])
            .and_where(Expr::col(VerifyIden::YtChId).eq(self.yt_ch_id.clone()))
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
        let user = Verify {
            yt_ch_id: String::from("test_id"),
            verification_code: String::from("test_code"),
            generated_at: Utc::now(),
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
        let mut u = Verify {
            yt_ch_id: String::from("test_id"),
            verification_code: String::from("test_code"),
            generated_at: Utc::now(),
        };
        u.insert(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let u0 = Verify::by_id(u.yt_ch_id.clone(), &tran)?.expect("no value");
        assert_eq!(u.yt_ch_id, u0.yt_ch_id);
        assert_eq!(u.verification_code, u0.verification_code);
        assert_eq!(u.generated_at, u0.generated_at);
        tran.finish()?;

        let tran = conn.transaction()?;
        u.verification_code = String::from("new_code");
        u.generated_at = Utc::now();
        u.update(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let u1 = Verify::by_id(u.yt_ch_id.clone(), &tran)?.expect("no value");

        tran.finish()?;
        assert_eq!(u.yt_ch_id, u1.yt_ch_id);
        assert_eq!(u.verification_code, u1.verification_code);
        assert_eq!(u.generated_at, u1.generated_at);
        Ok(())
    }
}
