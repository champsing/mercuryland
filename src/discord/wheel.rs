use poise;

use crate::{
    database::{self, wheel::Wheel},
    error::ServerError,
};

#[poise::command(slash_command)]
pub async fn fetch_wheel(
    ctx: super::Context<'_>,
    #[description = "The id of wheel session"] wheel_id: String,
) -> Result<(), ServerError> {
    let id = match u16::from_str_radix(&wheel_id, 16) {
        Err(_) => return Err(ServerError::Internal(String::from("Invalid wheel id"))),
        Ok(i) => i,
    };

    let content: Vec<String> = {
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
        serde_json::from_value(w.content)?
    };

    ctx.say(content.join("\n")).await?;
    Ok(())
}
