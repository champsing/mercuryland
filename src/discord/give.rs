use crate::{
    config::CONFIG,
    database::{self, user::Coin as CoinUser},
    error::ServerError,
};
use poise;

#[poise::command(slash_command)]
pub async fn give(
    ctx: super::Context<'_>,
    channel: String,
    amount: i64,
) -> Result<(), ServerError> {
    if CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        let mut display = String::new();
        {
            let mut connection = database::get_connection()?;
            let transaction = connection.transaction()?;
            if let Some(mut u) = CoinUser::by_youtube(channel.clone(), &transaction)? {
                display = u.display.clone();
                u.coin += amount;
                u.update(&transaction)?;
            }
            transaction.commit()?;
        }
        ctx.say(format!(
            "成功给 {} (`{}`) 添加了 {} 水星币",
            display, channel, amount
        ))
        .await?;
    } else {
        ctx.say("您没有权限").await?;
    }

    Ok(())
}
