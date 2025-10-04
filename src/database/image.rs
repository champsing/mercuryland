use crate::error::ServerError;
use rusqlite::{Row, Transaction};
use sea_query::{Expr, IdenStatic, Query, SqliteQueryBuilder, enum_def};
use sea_query_rusqlite::RusqliteBinder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Image {
    pub id: i64,
    pub name: Uuid,
    pub data: Vec<u8>,
    pub mime: String,
}

impl TryFrom<&Row<'_>> for Image {
    type Error = rusqlite::Error;

    fn try_from(value: &Row<'_>) -> Result<Self, Self::Error> {
        Ok(Image {
            id: value.get(ImageIden::Id.as_str())?,
            name: value.get(ImageIden::Name.as_str())?,
            data: value.get(ImageIden::Data.as_str())?,
            mime: value.get(ImageIden::Mime.as_str())?,
        })
    }
}

impl Image {
    pub fn insert(
        name: Uuid,
        data: Vec<u8>,
        mime: String,
        transaction: &Transaction,
    ) -> Result<i64, ServerError> {
        let (query, values) = Query::insert()
            .into_table(ImageIden::Table)
            .columns([ImageIden::Name, ImageIden::Data, ImageIden::Mime])
            .values([name.into(), data.into(), mime.into()])?
            .build_rusqlite(SqliteQueryBuilder);

        transaction.execute(&query, &*values.as_params())?;
        Ok(transaction.last_insert_rowid())
    }

    pub fn by_name(name: &Uuid, transaction: &Transaction) -> Result<Option<Self>, ServerError> {
        let (query, values) = Query::select()
            .columns([
                ImageIden::Id,
                ImageIden::Name,
                ImageIden::Data,
                ImageIden::Mime,
            ])
            .from(ImageIden::Table)
            .and_where(Expr::col(ImageIden::Name).eq(*name))
            .build_rusqlite(SqliteQueryBuilder);

        let mut stmt = transaction.prepare(&query)?;
        let result = stmt.query_map(&*values.as_params(), |row| Ok(Self::try_from(row)?))?;
        Ok(result.collect::<Result<Vec<_>, _>>()?.into_iter().next())
    }
}
