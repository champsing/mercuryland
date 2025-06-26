use crate::coin::command::CoinCommandManager;
use crate::coin::youtube::Coin;
use crate::config::CONFIG;
use crate::database::{coin::Coin as CoinUser, get_connection};
use crate::error::ServerError;
use poise::{self, CreateReply};
use rand::distributions::{Alphanumeric, DistString};
use serenity::all::{
    ChannelId, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateThread, EditMessage
};
use serenity::futures::lock::Mutex;
use std::sync::LazyLock;
use std::time::Duration;

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

pub enum CommandReply {
    Success,
    InvalidInput,
    InsufficientFunds,
    NoUserFound,
}

#[poise::command(slash_command)]
pub async fn overtime(
    ctx: super::Context<'_>,
    hours: f32,
    content: String,
) -> Result<(), ServerError> {
    let author_id = ctx.author().id.get();
    let channel_id = CONFIG.discord.exchange; // 水星交易所

    // 在一个同步块里处理所有 DB 逻辑，生成好要发送的 message
    let (reply, content) = 'ret: {
        if hours <= 0.0 {
            log::warn!("{}: incorrect hours", ctx.invocation_string());
            ctx.send(
                CreateReply::default()
                    .content("您輸入了無效的加班時數。請輸入大於 0 的正數（可以有小數）。")
                    .ephemeral(true),
            )
            .await?;
            break 'ret (CommandReply::InvalidInput, None);
        }

        let mut connection = get_connection()?;
        let transaction = connection.transaction()?;

        let record = Coin::by_discord(author_id.to_string(), &transaction)?;

        if record.is_none() {
            break 'ret (CommandReply::NoUserFound, None);
        }

        let mut record = record.unwrap();
        if record.discord_id == 0 {
            break 'ret (CommandReply::NoUserFound, None);
        }

        let cost = (hours * 1000.0).ceil() as i64; // 1000 水星幣每小時
        if record.coin < cost {
            break 'ret (CommandReply::InsufficientFunds, None);
        }

        let now = chrono::Utc::now();
        record.coin -= cost;
        record.updated_at = now;
        record.update(&transaction)?;

        println!(
            "[-] {} buy a(n) {}-hour overtime for {}",
            record.display, hours, content
        );

        transaction.commit()?;

        (
            CommandReply::Success,
            Some(format!(
                "\
# 加班台時數卡 #
## + {} 小時 ##
來自：{} (`{}`)
給惡靈的加班台留言：{}
如有疑義請在指令區使用 {} 執行退款流程。
",
                hours, record.display, record.id, content, CONFIG.slash_command_strings.refund_new
            )),
        )
    };

    match reply {
        CommandReply::Success => {
            ctx.send(CreateReply::default().content("交易成功！").ephemeral(true))
                .await?;
        }
        CommandReply::InvalidInput => {
            ctx.send(
                CreateReply::default()
                    .content("您輸入了無效的加班時數。請輸入大於 0 的正數（可以有小數）。")
                    .ephemeral(true),
            )
            .await?;
        }
        CommandReply::InsufficientFunds => {
            ctx.send(
                CreateReply::default()
                    .content(format!(
                        "**購買失敗。**\n您的水星幣不足以購買 {} 小時的加班台時數卡。您可以使用 {} 指令查詢。",
                        hours, CONFIG.slash_command_strings.coin
                    ))
                    .ephemeral(true),
            )
            .await?;
        }
        CommandReply::NoUserFound => {
            ctx.send(
                CreateReply::default()
                    .content(format!(
                        "查無資料，請先使用 {} 將 Discord 帳號關聯到您的 YouTube 頻道。",
                        CONFIG.slash_command_strings.link
                    ))
                    .ephemeral(true),
            )
            .await?;
        }
    }

    // 此处已经不再持有 rusqlite::Transaction，可以安全 .await
    if let Some(content) = content {
        let mut message = ChannelId::from(channel_id)
            .send_message(
                &ctx.serenity_context().http,
                CreateMessage::new()
                    .content(&content)
                    .button(CreateButton::new("refund").label("退款申请")),
            )
            .await?;

        let interaction = message
            .await_component_interaction(&ctx.serenity_context().shard)
            .timeout(Duration::from_secs(72 * 60 * 60)) // 72 hours
            .await;

        if let Some(interaction) = interaction {
            interaction
                .create_response(
                    &ctx,
                    // This time we dont edit the message but reply to it
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::default()
                            // Make the message hidden for other users by setting `ephemeral(true)`.
                            .ephemeral(true)
                            .content(format!("开始处理退款")),
                    ),
                )
                .await
                .unwrap();

            let author = ctx.author().clone();
            let case_number = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);

            let refund = ChannelId::new(CONFIG.discord.exchange) //CONFIG.discord.exchange 1248793225767026758
                .create_thread_from_message(
                    ctx.http(),
                    message.id,
                    CreateThread::new(format!("退款討論串 {}-{}", author.name, case_number)),
                )
                .await?;

            let content = format!(
"\
申請人 <@{}>
案號 {}
YouTube 頻道 ID：{}
請說明您希望退款的理由。
=============在這則訊息以下開始處理退款=============
",
                author.id.get(), case_number, "1234"
            );
    
            refund.send_message(&ctx.serenity_context().http, CreateMessage::new().content(content)).await?;
        }

        message
            .edit(
                &ctx.serenity_context().http,
                EditMessage::new().components(vec![]),
            )
            .await?;
    }

    Ok(())
}
