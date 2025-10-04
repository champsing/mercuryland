use crate::{
    database::{self, user::Coin as CoinUser},
    error::ServerError,
};
use poise;

#[poise::command(slash_command)]
pub async fn coin(
    ctx: super::Context<'_>,
    #[description = "YouTube channel ID, use your Discord account if ignored"]
    #[description_localized("zh-TW", "YouTube 頻道 ID，若忽略則使用您的 Discord 帳號")]
    channel: Option<String>,
) -> Result<(), ServerError> {
    let coin = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        match channel {
            Some(c) => match CoinUser::by_youtube(c, &transaction)? {
                None => 0,
                Some(u) => u.coin,
            },
            None => match CoinUser::by_discord(ctx.author().id.get().to_string(), &transaction)? {
                None => 0,
                Some(u) => u.coin,
            },
        }
    };

    ctx.say(format!("您目前有 {} 水星幣。", coin)).await?;
    Ok(())
}
