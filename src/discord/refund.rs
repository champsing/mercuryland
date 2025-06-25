use crate::{coin::youtube::Coin, config::CONFIG, database, discord, error::ServerError};
use chrono::Utc;
use poise::{
    self,
    serenity_prelude::{ChannelId, CreateMessage, CreateThread},
};
use rand::distributions::{Alphanumeric, DistString};
use serenity::all::EditThread;

#[poise::command(slash_command)]
pub async fn refund(ctx: super::Context<'_>) -> Result<(), ServerError> {
    ctx.say("Refund Entrypoint.").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn new(ctx: super::Context<'_>) -> Result<(), ServerError> {
    let author = ctx.author().clone();

    let case_number = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);

    let refund_thread = ChannelId::new(CONFIG.discord.exchange) //CONFIG.discord.exchange 1248793225767026758
        .create_thread(
            ctx.http(),
            CreateThread::new(format!("退款討論串 {}-{}", author.name, case_number)),
        )
        .await?;

    let youtube_id = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        match Coin::by_discord(ctx.author().id.get().to_string(), &transaction)? {
            Some(u) => u.id,
            None => "查無資料，請於下方提供您的 YouTube 頻道 ID 以利作業。".to_string(),
        }
    };
    let content = format!(
        "\
申請人 <@{}>
案號 {}
YouTube 頻道 ID：{}
請說明您希望退款的理由。
=============在這則訊息以下開始處理退款=============
",
        author.id.get(), case_number, youtube_id
    );
    
    discord::Receiver::ChannelId(refund_thread.id.get())
        .message(CreateMessage::new().content(content))
        .await?;

    ctx.reply(format!("<#{}>", refund_thread.id)).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn close(ctx: super::Context<'_>) -> Result<(), ServerError> {
    if CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        let channel_name = ctx.channel_id().name(ctx.http()).await?.split_off(16);

        let now = Utc::now().format("%Y/%m/%d %H:%M");

        let content = format!(
            "\
討論串 {} 於 {} 處理完成。
如果需要重啟討論，請私訊管理員。
=============本案號已結案，將討論串存檔，請勿新增更多訊息=============",
            channel_name, now
        );

        ctx.reply(content).await?;

        ctx.channel_id()
            .edit_thread(ctx.http(), EditThread::new().archived(true).locked(true))
            .await?;
    } else {
        ctx.say("您沒有權限使用此指令").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn reopen(ctx: super::Context<'_>) -> Result<(), ServerError> {
    if CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        ctx.channel_id()
            .edit_thread(ctx.http(), EditThread::new().archived(false).locked(false))
            .await?;

        let channel_name = ctx.channel_id().name(ctx.http()).await?.split_off(16);

        let now = Utc::now().format("%Y/%m/%d %H:%M");

        let content = format!(
            "\
討論串 {} 於 {} 再開。
請繼續處理退款事宜。
=============在這則訊息以下開始處理退款=============",
            channel_name, now
        );

        ctx.reply(content).await?;


    } else {
        ctx.say("您沒有權限使用此指令").await?;
    }

    Ok(())
}
