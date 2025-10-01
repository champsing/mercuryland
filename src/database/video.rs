use crate::error::ServerError;
use chrono::NaiveDate;
use rusqlite::{Row, Transaction, types::Type};
use sea_query::{Expr, IdenStatic, Query, SqliteQueryBuilder, enum_def};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Video {
    pub id: Option<i64>,
    pub date: NaiveDate,
    pub link: String,
    pub title: String,
    pub tags: Vec<String>,
    pub duration: String,
}

impl TryFrom<&Row<'_>> for Video {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        let tags_json: String = value.get(VideoIden::Tags.as_str())?;
        let tags: Vec<String> = from_str(&tags_json).map_err(|err| {
            rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err))
        })?;

        let date_str: String = value.get(VideoIden::Date.as_str())?;
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|err| {
            rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err))
        })?;

        Ok(Self {
            id: Some(value.get(VideoIden::Id.as_str())?),
            date,
            link: value.get(VideoIden::Link.as_str())?,
            title: value.get(VideoIden::Title.as_str())?,
            tags,
            duration: value.get(VideoIden::Duration.as_str())?,
        })
    }
}

impl Video {
    pub fn insert(&mut self, transaction: &Transaction) -> Result<(), ServerError> {
        assert!(self.id.is_none(), "video.id must be None before insert");
        let tags_json = to_string(&self.tags)?;
        let (query, values) = Query::insert()
            .into_table(VideoIden::Table)
            .columns([
                VideoIden::Date,
                VideoIden::Link,
                VideoIden::Title,
                VideoIden::Tags,
                VideoIden::Duration,
            ])
            .values([
                self.date.format("%Y-%m-%d").to_string().into(),
                self.link.clone().into(),
                self.title.clone().into(),
                tags_json.into(),
                self.duration.clone().into(),
            ])?
            .build_rusqlite(SqliteQueryBuilder);

        transaction.execute(&query, &*values.as_params())?;

        self.id = Some(transaction.last_insert_rowid());

        Ok(())
    }

    pub fn all(transaction: &Transaction) -> Result<Vec<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                VideoIden::Id,
                VideoIden::Date,
                VideoIden::Link,
                VideoIden::Title,
                VideoIden::Tags,
                VideoIden::Duration,
            ])
            .from(VideoIden::Table)
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let videos = statement
            .query_and_then(&*values.as_params(), |row| Video::try_from(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(videos)
    }

    pub fn find_by_link(
        link: &str,
        transaction: &Transaction,
    ) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                VideoIden::Id,
                VideoIden::Date,
                VideoIden::Link,
                VideoIden::Title,
                VideoIden::Tags,
                VideoIden::Duration,
            ])
            .from(VideoIden::Table)
            .and_where(Expr::col(VideoIden::Link).eq(link))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let video = statement
            .query_and_then(&*values.as_params(), |row| Video::try_from(row))?
            .next();

        Ok(video.transpose()?)
    }

    pub fn find_by_id(id: i64, transaction: &Transaction) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                VideoIden::Id,
                VideoIden::Date,
                VideoIden::Link,
                VideoIden::Title,
                VideoIden::Tags,
                VideoIden::Duration,
            ])
            .from(VideoIden::Table)
            .and_where(Expr::col(VideoIden::Id).eq(id))
            .build_rusqlite(SqliteQueryBuilder);

        let mut statement = transaction.prepare(&query)?;
        let video = statement
            .query_and_then(&*values.as_params(), |row| Video::try_from(row))?
            .next();

        Ok(video.transpose()?)
    }

    pub fn delete(&self, transaction: &Transaction) -> Result<usize, ServerError> {
        let mut query = Query::delete();
        query.from_table(VideoIden::Table);

        if let Some(id) = self.id {
            query.and_where(Expr::col(VideoIden::Id).eq(id));
        } else {
            query.and_where(Expr::col(VideoIden::Link).eq(self.link.clone()));
        }

        let (query, values) = query.build_rusqlite(SqliteQueryBuilder);
        let affected = transaction.execute(&query, &*values.as_params())?;
        Ok(affected as usize)
    }

    pub fn update(&self, transaction: &Transaction) -> Result<usize, ServerError> {
        let tags_json = to_string(&self.tags)?;

        let mut query = Query::update();
        query.table(VideoIden::Table).values([
            (
                VideoIden::Date,
                self.date.format("%Y-%m-%d").to_string().into(),
            ),
            (VideoIden::Title, self.title.clone().into()),
            (VideoIden::Tags, tags_json.into()),
            (VideoIden::Duration, self.duration.clone().into()),
        ]);

        if let Some(id) = self.id {
            query.and_where(Expr::col(VideoIden::Id).eq(id));
        } else {
            query.and_where(Expr::col(VideoIden::Link).eq(self.link.clone()));
        }

        let (query, values) = query.build_rusqlite(SqliteQueryBuilder);
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
        let mut video = Video {
            id: None,
            date: NaiveDate::from_ymd_opt(2020, 3, 8).expect("valid date"),
            link: "video_link".into(),
            title: "Some title".into(),
            tags: vec!["tag1".into(), "tag2".into()],
            duration: "00:10:00".into(),
        };

        let tran = conn.transaction()?;
        video.insert(&tran)?;
        let id = video.id.expect("id set");
        tran.commit()?;

        let tran = conn.transaction()?;
        let fetched = Video::find_by_link(&video.link, &tran)?.expect("video");
        let fetched_by_id = Video::find_by_id(id, &tran)?.expect("video");
        assert_eq!(Some(id), fetched.id);
        assert_eq!(Some(id), fetched_by_id.id);
        assert_eq!(video.link, fetched_by_id.link);
        assert_eq!(video.title, fetched.title);
        assert_eq!(video.title, fetched_by_id.title);
        assert_eq!(video.tags, fetched.tags);
        assert_eq!(video.tags, fetched_by_id.tags);
        assert_eq!(video.date, fetched.date);
        assert_eq!(video.date, fetched_by_id.date);
        tran.finish()?;

        Ok(())
    }

    #[test]
    fn list_delete_and_update() -> Result<(), ServerError> {
        let mut conn = setup_conn()?;
        let mut first = Video {
            id: None,
            date: NaiveDate::from_ymd_opt(2020, 3, 8).expect("valid date"),
            link: "first".into(),
            title: "First".into(),
            tags: vec!["a".into()],
            duration: "00:01:00".into(),
        };
        let mut second = Video {
            id: None,
            date: NaiveDate::from_ymd_opt(2020, 4, 1).expect("valid date"),
            link: "second".into(),
            title: "Second".into(),
            tags: vec!["b".into()],
            duration: "00:02:00".into(),
        };

        let tran = conn.transaction()?;
        first.insert(&tran)?;
        second.insert(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let mut all = Video::all(&tran)?;
        all.sort_by_key(|v| v.link.clone());
        assert_eq!(all.len(), 2);
        tran.finish()?;

        let tran = conn.transaction()?;
        first.title = String::from("First Updated");
        first.tags = vec![String::from("x")];
        first.duration = String::from("00:03:00");
        first.date = NaiveDate::from_ymd_opt(2020, 5, 1).expect("valid date");
        let updated = first.update(&tran)?;
        assert_eq!(updated, 1);
        tran.commit()?;

        let tran = conn.transaction()?;
        let fetched = Video::find_by_link("first", &tran)?.expect("video");
        assert_eq!(fetched.tags, vec!["x".to_string()]);
        assert_eq!(fetched.date, first.date);
        assert_eq!(fetched.title, first.title);
        assert_eq!(fetched.duration, first.duration);
        tran.finish()?;

        let tran = conn.transaction()?;
        let deleted = second.delete(&tran)?;
        assert_eq!(deleted, 1);
        tran.commit()?;

        let tran = conn.transaction()?;
        let all = Video::all(&tran)?;
        assert_eq!(all.len(), 1);
        tran.finish()?;

        Ok(())
    }
}
