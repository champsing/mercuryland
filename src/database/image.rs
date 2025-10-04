use rusqlite::Row;
use sea_query::{IdenStatic, enum_def};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_def]
pub struct Image {
    pub id: Option<i64>,
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