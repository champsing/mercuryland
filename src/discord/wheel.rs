use std::iter::once;

use poise;

use crate::{
    database::{self, wheel::Wheel},
    error::ServerError,
};
use itertools::Itertools;

#[poise::command(slash_command)]
pub async fn fetch_wheel(
    ctx: super::Context<'_>,
    #[description = "The id of wheel session"] wheel_id: String,
) -> Result<(), ServerError> {
    let id = match u16::from_str_radix(&wheel_id, 16) {
        Err(_) => return Err(ServerError::Internal(String::from("Invalid wheel id"))),
        Ok(i) => i,
    };

    let (time, content): (_, Vec<String>) = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        let w = match Wheel::by_id(id, &transaction)? {
            None => {
                return Err(ServerError::Internal(String::from(
                    "Wheel id does not exists",
                )))
            }
            Some(w) => w,
        };
        (
            w.updated_at.timestamp(),
            serde_json::from_value(w.content).expect("Can't find the content"),
        )
    };

    let content = content
        .into_iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i, s));

    let message = once(format!("<t:{}:D>", time)).chain(content).join("\n");

    ctx.say(message).await?;
    Ok(())
}
