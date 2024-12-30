use poise;
use crate::{
    database::{self, coin::Coin as CoinUser},
    error::ServerError,
};

#[poise::command(slash_command)]
pub async fn coin(
    ctx: super::Context<'_>,
    #[description = "The channel id of youtube account"] channel: String,
) -> Result<(), ServerError> {
    let coin = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        match CoinUser::by_id(channel, &transaction)? {
            None => 0,
            Some(u) => u.coin,
        }
    };

    ctx.say(format!("{}", coin)).await?;
    Ok(())
}
