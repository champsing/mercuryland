use crate::coin::command::CoinCommandManager;
use crate::coin::youtube::Coin;
use crate::config::CONFIG;
use crate::database::{coin::Coin as CoinUser, get_connection};

use crate::discord;
use crate::error::ServerError;

use poise::{self, CreateReply};
use serenity::all::CreateMessage;
use serenity::futures::lock::Mutex;
use std::sync::LazyLock;

fn find_coin_user(msg_author_id: String) -> Result<Option<String>, ServerError> {
    let mut connection = get_connection()?;
    let transaction = connection.transaction()?;
    let user_id = match CoinUser::by_discord(msg_author_id, &transaction)? {
        Some(u) => Some(u.id),
        None => None,
    };
    Ok(user_id)
}

#[poise::command(slash_command)]
pub async fn purchase(ctx: super::Context<'_>) -> Result<(), ServerError> {
    ctx.say("Purchase Entrypoint.").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn booster(
    ctx: super::Context<'_>,
    amp: u32,
    penalty_content: String,
) -> Result<(), ServerError> {
    // if amp = 0 or > 9
    if amp == 0 || amp > 9 {
        log::warn!("{}: incorrect level", ctx.invocation_string());
        ctx.send(
            CreateReply::default()
                .content("您輸入了無效的加倍倍率。請輸入 1 到 9 之間的整數。")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let author = ctx.author();

    //get Coin user by discord id
    let author_youtube_channel_id = {
        let user_id = match find_coin_user(author.id.get().to_string())? {
            Some(id) => id,
            None => {
                ctx.send(
                    CreateReply::default()
                        .content(format!(
                            "查無資料，請先使用 {} 將 Discord 帳號關聯到您的 YouTube 頻道。",
                            CONFIG.slash_command_strings.link
                        ))
                        .ephemeral(true),
                )
                .await?;
                String::from("No user found")
            }
        };

        if user_id == "No user found".to_string() {
            return Ok(());
        } else {
            user_id
        }
    };

    static CONTEXT: LazyLock<Mutex<CoinCommandManager>> =
        LazyLock::new(|| Mutex::new(CoinCommandManager::new()));

    let manager = CONTEXT.lock().await;
    let now = chrono::Utc::now();

    manager
        .buy_booster(
            &author_youtube_channel_id,
            amp.into(),
            &penalty_content,
            now,
        )
        .await?;
    ctx.send(
        CreateReply::default()
            .content(format!("已發送購買請求，請等待處理。"))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn overtime(
    ctx: super::Context<'_>,
    hours: u32,
    content: String,
) -> Result<(), ServerError> {
    if hours == 0 || hours > 24 {
        log::warn!("{}: incorrect hours", ctx.invocation_string());
        ctx.send(
            CreateReply::default()
                .content("您輸入了無效的加班時數。請輸入 1 到 24 之間的整數。")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let author_id = ctx.author().id.get();

    let channel_id: u64 = CONFIG.discord.exchange; // 水星交易所

    // 在一个同步块里处理所有 DB 逻辑，生成好要发送的 message
    let maybe_message = {
        let mut connection = get_connection()?;
        let transaction = connection.transaction()?;

        // 如果用户存在且有足够的 coin，就更新并准备 message
        let msg = if let Some(mut record) = Coin::by_discord(author_id.to_string(), &transaction)? {
            let cost = (hours * 1000).into(); // 1000 水星幣每小時
            let now = chrono::Utc::now();
            if record.coin >= cost {
                record.coin -= cost;
                record.updated_at = now;
                record.update(&transaction)?;

                println!(
                    "[-] {} buy a(n) {}-hour overtime for {}",
                    record.display, hours, content
                );

                Some(format!(
                    "\
# 加班台時數卡 #
## {} 小時 ##
來自：{} (`{}`)
給惡靈的加班台留言：{}
如有疑義請在指令區使用 {} 執行退款流程。
",
                    hours, record.display, record.id, content, CONFIG.slash_command_strings.refund_new
                ))
            } else {
                // coin 不足或其他情况
                Some(format!(
                    "您的水星幣不足以購買 {} 小時的加班台時數卡。您可以使用 {} 指令查詢。",
                    hours, CONFIG.slash_command_strings.coin
                ))
            }
        } else {
            None
        };

        transaction.commit()?;
        msg
    };

    // 此处已经不再持有 rusqlite::Transaction，可以安全 .await
    if let Some(content) = maybe_message {
        discord::Receiver::ChannelId(channel_id)
            .message(CreateMessage::new().content(content))
            .await?;
    }

    Ok(())
}
