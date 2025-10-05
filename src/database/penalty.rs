use crate::error::ServerError;
use chrono::NaiveDate;
use rusqlite::{Row, Transaction, types::Type};
use sea_query::{Expr, IdenStatic, Query, SqliteQueryBuilder, enum_def};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PenaltyState {
    Inactive = 0,
    NotStarted = 1,
    InProgress = 2,
    BarelyDone = 3,
    Completed = 4,
}

impl From<i32> for PenaltyState {
    fn from(value: i32) -> Self {
        match value {
            0 => PenaltyState::Inactive,
            1 => PenaltyState::NotStarted,
            2 => PenaltyState::InProgress,
            3 => PenaltyState::BarelyDone,
            4 => PenaltyState::Completed,
            _ => PenaltyState::Inactive, // Default to Inactive for invalid values
        }
    }
}

impl From<PenaltyState> for i32 {
    fn from(state: PenaltyState) -> i32 {
        state as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Penalty {
    pub id: i64,
    pub date: NaiveDate,
    pub name: String,
    pub detail: String,
    pub state: PenaltyState,
    pub history: Vec<(PenaltyState, NaiveDate)>,
}

impl TryFrom<&Row<'_>> for Penalty {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        let history_json: String = value.get(PenaltyIden::History.as_str())?;
        let history: Vec<(i32, NaiveDate)> = from_str(&history_json).map_err(|err| {
            rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err))
        })?;
        
        // Convert i32 values to PenaltyState in history
        let history: Vec<(PenaltyState, NaiveDate)> = history.into_iter()
            .map(|(state_num, date)| (PenaltyState::from(state_num), date))
            .collect();

        let state_num: i32 = value.get(PenaltyIden::State.as_str())?;
        let state = PenaltyState::from(state_num);

        let date_str: String = value.get(PenaltyIden::Date.as_str())?;
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|err| {
            rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err))
        })?;

        Ok(Self {
            id: value.get(PenaltyIden::Id.as_str())?,
            date,
            name: value.get(PenaltyIden::Name.as_str())?,
            detail: value.get(PenaltyIden::Detail.as_str())?,
            state,
            history,
        })
    }
}

impl Penalty {
    pub fn insert(&mut self, transaction: &Transaction) -> Result<(), ServerError> {
        let history_json = to_string(&self.history.iter().map(|(state, date)| (i32::from(*state), *date)).collect::<Vec<_>>())?;
        let (query, values) = Query::insert()
            .into_table(PenaltyIden::Table)
            .columns([
                PenaltyIden::Date,
                PenaltyIden::Name,
                PenaltyIden::Detail,
                PenaltyIden::State,
                PenaltyIden::History,
            ])
            .values([
                self.date.format("%Y-%m-%d").to_string().into(),
                self.name.clone().into(),
                self.detail.clone().into(),
                i32::from(self.state).into(),
                history_json.into(),
            ])?
            .build_rusqlite(SqliteQueryBuilder);

        transaction.execute(&query, &*values.as_params())?;

        self.id = transaction.last_insert_rowid();

        Ok(())
    }

    pub fn all(transaction: &Transaction) -> Result<Vec<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                PenaltyIden::Id,
                PenaltyIden::Date,
                PenaltyIden::Name,
                PenaltyIden::Detail,
                PenaltyIden::State,
                PenaltyIden::History,
            ])
            .from(PenaltyIden::Table)
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let penalties = statement
            .query_and_then(&*values.as_params(), |row| Penalty::try_from(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(penalties)
    }

    pub fn by_id(id: i64, transaction: &Transaction) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                PenaltyIden::Id,
                PenaltyIden::Date,
                PenaltyIden::Name,
                PenaltyIden::Detail,
                PenaltyIden::State,
                PenaltyIden::History,
            ])
            .from(PenaltyIden::Table)
            .and_where(Expr::col(PenaltyIden::Id).eq(id))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let penalty = statement
            .query_and_then(&*values.as_params(), |row| Penalty::try_from(row))?
            .next();

        Ok(penalty.transpose()?)
    }

    pub fn update(&self, transaction: &Transaction) -> Result<usize, ServerError> {
        let history_json = to_string(&self.history.iter().map(|(state, date)| (i32::from(*state), *date)).collect::<Vec<_>>())?;

        let (query, values) = Query::update()
            .table(PenaltyIden::Table)
            .values([
                (
                    PenaltyIden::Date,
                    self.date.format("%Y-%m-%d").to_string().into(),
                ),
                (PenaltyIden::Name, self.name.clone().into()),
                (PenaltyIden::Detail, self.detail.clone().into()),
                (PenaltyIden::State, i32::from(self.state).into()),
                (PenaltyIden::History, history_json.into()),
            ])
            .and_where(Expr::col(PenaltyIden::Id).eq(self.id))
            .build_rusqlite(SqliteQueryBuilder);
        let affected = transaction.execute(&query, &*values.as_params())?;
        Ok(affected as usize)
    }

    pub fn delete(&self, transaction: &Transaction) -> Result<usize, ServerError> {
        let (query, values) = Query::delete()
            .from_table(PenaltyIden::Table)
            .and_where(Expr::col(PenaltyIden::Id).eq(self.id))
            .build_rusqlite(SqliteQueryBuilder);

        let affected = transaction.execute(&query, &*values.as_params())?;
        Ok(affected as usize)
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
        let mut penalty = Penalty {
            id: 0,
            date: NaiveDate::from_ymd_opt(2025, 10, 5).expect("valid date"),
            name: "Test Penalty".into(),
            detail: "<p>Test detail</p>".into(),
            state: PenaltyState::NotStarted,
            history: vec![],
        };

        let tran = conn.transaction()?;
        penalty.insert(&tran)?;
        let id = penalty.id;
        tran.commit()?;

        let tran = conn.transaction()?;
        let fetched = Penalty::by_id(id, &tran)?.expect("penalty");
        assert_eq!(id, fetched.id);
        assert_eq!(penalty.name, fetched.name);
        assert_eq!(penalty.detail, fetched.detail);
        assert_eq!(penalty.state, fetched.state);
        assert_eq!(penalty.history, fetched.history);
        assert_eq!(penalty.date, fetched.date);
        tran.finish()?;

        Ok(())
    }

    #[test]
    fn list_and_delete() -> Result<(), ServerError> {
        let mut conn = setup_conn()?;
        let mut first = Penalty {
            id: 0,
            date: NaiveDate::from_ymd_opt(2025, 10, 1).expect("valid date"),
            name: "First Penalty".into(),
            detail: "<p>First detail</p>".into(),
            state: PenaltyState::Inactive,
            history: vec![],
        };
        let mut second = Penalty {
            id: 0,
            date: NaiveDate::from_ymd_opt(2025, 10, 2).expect("valid date"),
            name: "Second Penalty".into(),
            detail: "<p>Second detail</p>".into(),
            state: PenaltyState::InProgress,
            history: vec![],
        };

        let tran = conn.transaction()?;
        first.insert(&tran)?;
        second.insert(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let all = Penalty::all(&tran)?;
        assert_eq!(all.len(), 2);
        tran.finish()?;

        let tran = conn.transaction()?;
        let deleted = second.delete(&tran)?;
        assert_eq!(deleted, 1);
        tran.commit()?;

        let tran = conn.transaction()?;
        let all = Penalty::all(&tran)?;
        assert_eq!(all.len(), 1);
        tran.finish()?;

        Ok(())
    }
}