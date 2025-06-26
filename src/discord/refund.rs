use crate::{config::CONFIG, error::ServerError};
use chrono::Utc;
use poise::{self};
use serenity::all::EditThread;

#[poise::command(slash_command)]
pub async fn refund(ctx: super::Context<'_>) -> Result<(), ServerError> {
    ctx.say("Refund Entrypoint.").await?;
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
