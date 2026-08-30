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
    pub fn insert(&mut self, transaction: &Transaction) -> Result<(), ServerError> {
        // Create a UUIDv5 based on the image data to ensure uniqueness
        assert!(self.name == Uuid::new_v5(&Uuid::NAMESPACE_OID, self.data.as_slice()));

        let (query, values) = Query::insert()
            .into_table(ImageIden::Table)
            .columns([ImageIden::Name, ImageIden::Data, ImageIden::Mime])
            .values([
                self.name.into(),
                self.data.clone().into(),
                self.mime.clone().into(),
            ])?
            .build_rusqlite(SqliteQueryBuilder);

        transaction.execute(&query, &*values.as_params())?;
        Ok(())
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
    fn insert_and_find_by_name() -> Result<(), ServerError> {
        let mut conn = setup_conn()?;

        let data = b"some image bytes".to_vec();
        // `insert` asserts the name is a UUIDv5 derived from the data.
        let name = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, data.as_slice());
        let mut image = Image {
            id: 0,
            name,
            data,
            mime: "image/png".into(),
        };

        let tran = conn.transaction()?;
        image.insert(&tran)?;
        tran.commit()?;

        let tran = conn.transaction()?;
        let fetched = Image::by_name(&name, &tran)?.expect("image should be found");
        assert_eq!(fetched.name, name);
        assert_eq!(fetched.data, image.data);
        assert_eq!(fetched.mime, image.mime);
        tran.finish()?;

        Ok(())
    }

    #[test]
    fn by_name_not_found() -> Result<(), ServerError> {
        let mut conn = setup_conn()?;

        let tran = conn.transaction()?;
        let missing = uuid::Uuid::new_v4();
        let result = Image::by_name(&missing, &tran)?;
        assert!(result.is_none());
        tran.finish()?;

        Ok(())
    }
}
