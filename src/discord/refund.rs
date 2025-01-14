use crate::{
    coin::youtube::Coin, config::CONFIG, database, discord, error::ServerError
};
use poise::{self, serenity_prelude::{CreateMessage, ChannelId, CreateThread}};
use rand::distributions::{Alphanumeric, DistString};

#[poise::command(slash_command)]
pub async fn refund(
    ctx: super::Context<'_>,
) -> Result<(), ServerError> {
    ctx.say(ctx.id().to_string()).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn new(
    ctx: super::Context<'_>,
) -> Result<(), ServerError> {

    // ctx.reply("現在開始連結 Discord 帳號至 YouTube 頻道。請依照私訊說明操作。").await?;
    let author = ctx.author().name.clone();

    let case_number = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);

    let refund_thread = ChannelId::new(CONFIG.discord.exchange).create_thread(ctx.http(), CreateThread::new(format!("退款討論串 {}-{}", author, case_number))).await?;

    let youtube_id = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        match Coin::by_discord(ctx.author().id.get().to_string(), &transaction)? {
            Some(u) => u.id,
            None => "查無資料，請於下方提供您的 YouTube 頻道 ID 以利作業。".to_string(),
        }
    };
    let content = format!("\
申請人 {}
案號 {}
YouTube 頻道 ID：{}
請說明您希望退款的理由
=============在這則訊息以下開始處理退款=============
", author, case_number, youtube_id);
    
    discord::Receiver::ChannelId(refund_thread.id.get())
        .message(CreateMessage::new().content(content))
        .await?;

    ctx.reply(format!("<#{}>", refund_thread.id)).await?;

    Ok(())
}

// #[poise::command(slash_command)]
// pub async fn close(
//     ctx: super::Context<'_>,
// ) -> Result<(), ServerError> {
//     Ok(())
// }
