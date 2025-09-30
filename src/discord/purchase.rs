use crate::coin::command::CoinCommandManager;
use crate::coin::youtube::Coin;
use crate::config::CONFIG;
use crate::database::{coin::Coin as CoinUser, get_connection};
use crate::error::ServerError;
use chrono::Days;
use poise::{self, CreateReply};
use rand::distributions::{Alphanumeric, DistString};
use serenity::all::{
    ChannelId, CreateActionRow, CreateButton, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, CreateThread, EditMessage,
};
use std::time::Duration;

pub enum CommandReply {
    Success,
    InvalidInput,
    InsufficientFunds,
    NoUserFound,
}

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
    let author = ctx.author();
    let channel_id = CONFIG.discord.exchange;

    // 在一个同步块里处理所有 DB 逻辑，生成好要发送的 message
    let (reply, content) = 'ret: {
        if amp <= 0 || amp > 9 {
            log::warn!("{}: incorrect level", ctx.invocation_string());
            break 'ret (CommandReply::InvalidInput, None);
        }

        let mut connection = get_connection()?;
        let transaction = connection.transaction()?;

        let user_id = match find_coin_user(author.id.get().to_string())? {
            Some(uid) => uid,
            None => String::from("User not found"),
        };

        let mut record = match Coin::by_youtube(user_id, &transaction)? {
            Some(r) => r,
            None => {
                break 'ret (CommandReply::NoUserFound, None);
            }
        };

        if record.discord_id == 0 {
            break 'ret (CommandReply::NoUserFound, None);
        }

        let coin_config = CoinCommandManager::new().config;
        let cost = coin_config.booster_cost(amp.into());

        if record.coin < cost {
            break 'ret (CommandReply::InsufficientFunds, None);
        }
        let now = chrono::Utc::now();
        let due = now.checked_add_days(Days::new(3)).unwrap();

        record.coin -= cost;
        record.updated_at = now;
        record.update(&transaction)?;

        println!(
            "[-] {} buy a level-{} booster for {}",
            record.display, amp, penalty_content
        );

        transaction.commit()?;

        (
            CommandReply::Success,
            Some(format!(
                "\
# 懲罰加倍 #
> ## 「{}」 x **{}** 倍 ##
- 來自： {} (`{}`)，結餘 **{}** 水星幣。
-# 如有疑義，或未抽中想領取半價退款，請在 72 小時內（<t:{}:f> 之前）執行退款流程。
",
                penalty_content,
                amp,
                record.display,
                record.id,
                record.coin,
                due.timestamp()
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
                    .content("您輸入了無效的倍率。請輸入 1 ~ 9 的整數。")
                    .ephemeral(true),
            )
            .await?;
        }
        CommandReply::InsufficientFunds => {
            ctx.send(
                CreateReply::default()
                    .content(
                        format!(
                            "**購買失敗。**\n您的水星幣不足以購買 x{} 的加倍倍率。您可以使用 {} 指令查詢餘額。",
                            amp,
                            super::command_mentions::get("coin").unwrap_or("/coin")
                        )
                    )
                    .ephemeral(true)
            ).await?;
        }
        CommandReply::NoUserFound => {
            ctx.send(
                CreateReply::default()
                    .content(
                        format!(
                            "**找不到您的 Discord 用戶記錄。**\n請先使用 {} 將 Discord 帳號關聯到您的 YouTube 頻道，才能在 Discord 使用水星幣購買。",
                            super::command_mentions::get("link").unwrap_or("/link")
                        )
                    )
                    .ephemeral(true)
            ).await?;
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
            .timeout(Duration::from_secs(72 * 60 * 60))
            .await; // 72 level

        if let Some(interaction) = interaction {
            // if not the buyer clicking button
            if interaction.user.id.get() != author.id.get().clone() {
                ctx.send(
                    CreateReply::default()
                        .content(format!(
                            "**您並非這筆訂單的購買者。**您僅有權限退款自己的訂單。"
                        ))
                        .ephemeral(true),
                )
                .await?;
                return Ok(());
            }

            message
                .edit(
                    &ctx.serenity_context().http,
                    EditMessage::new().components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new("refund")
                            .label("已申請退款")
                            .disabled(true),
                    ])]),
                )
                .await?;

            let _ = match interaction
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
            {
                Ok(_) => (),
                Err(err) => {
                    ctx.send(
                        CreateReply::default()
                            .content(format!(
                                "**退款流程啟動失敗。**請稍後再試一次。\n-# Error：{}",
                                err
                            ))
                            .ephemeral(true),
                    )
                    .await?;
                    return Ok(());
                }
            };

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
退款商品類型：懲罰加倍卡
申請人 <@{}>
案號 {}
請說明您希望退款的理由。
=============在這則訊息以下開始處理退款=============
",
                author.id.get(),
                case_number
            );

            refund
                .send_message(
                    &ctx.serenity_context().http,
                    CreateMessage::new().content(content),
                )
                .await?;
        }
    }

    Ok(())
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
            break 'ret (CommandReply::InvalidInput, None);
        }

        let mut connection = get_connection()?;
        let transaction = connection.transaction()?;

        let mut record = match Coin::by_discord(author_id.to_string(), &transaction)? {
            Some(r) => r,
            None => {
                break 'ret (CommandReply::NoUserFound, None);
            }
        };

        if record.discord_id == 0 {
            break 'ret (CommandReply::NoUserFound, None);
        }

        let cost = (hours * 1000.0).ceil() as i64; // 1000 水星幣每小時
        if record.coin < cost {
            break 'ret (CommandReply::InsufficientFunds, None);
        }

        let now = chrono::Utc::now();
        let due = now.checked_add_days(Days::new(3)).unwrap();

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
- 來自：{} (`{}`)，結餘 **{}** 水星幣。
- 給惡靈的加班台留言：
> {}
-# 如有疑義請在 72 小時內（<t:{}:f> 之前）執行退款流程。
",
                hours,
                record.display,
                record.id,
                record.coin,
                content,
                due.timestamp()
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
                    .content("您輸入了無效的時數。請輸入大於 0 的正數（可以有小數）。")
                    .ephemeral(true),
            )
            .await?;
        }
        CommandReply::InsufficientFunds => {
            ctx.send(
                CreateReply::default()
                    .content(
                        format!(
                            "**購買失敗。**\n您的水星幣不足以購買 {} 小時的加班台時數卡。您可以使用 {} 指令查詢餘額。",
                            hours,
                            super::command_mentions::get("coin").unwrap_or("/coin")
                        )
                    )
                    .ephemeral(true)
            ).await?;
        }
        CommandReply::NoUserFound => {
            ctx.send(
                CreateReply::default()
                    .content(
                        format!(
                            "**找不到您的 Discord 用戶記錄。**\n請先使用 {} 將 Discord 帳號關聯到您的 YouTube 頻道，才能在 Discord 使用水星幣購買。",
                            super::command_mentions::get("link").unwrap_or("/link")
                        )
                    )
                    .ephemeral(true)
            ).await?;
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
            .timeout(Duration::from_secs(72 * 60 * 60))
            .await; // 72 hours

        if let Some(interaction) = interaction {
            // if not the buyer clicking button
            if interaction.user.id.get() != author_id.clone() {
                ctx.send(
                    CreateReply::default()
                        .content(format!(
                            "**您並非這筆訂單的購買者。**您僅有權限退款自己的訂單。"
                        ))
                        .ephemeral(true),
                )
                .await?;
                return Ok(());
            }

            message
                .edit(
                    &ctx.serenity_context().http,
                    EditMessage::new().components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new("refund")
                            .label("已申請退款")
                            .disabled(true),
                    ])]),
                )
                .await?;

            let _ = match interaction
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
            {
                Ok(_) => (),
                Err(err) => {
                    ctx.send(
                        CreateReply::default()
                            .content(format!(
                                "**退款流程啟動失敗。**請稍後再試一次。\n-# Error：{}",
                                err
                            ))
                            .ephemeral(true),
                    )
                    .await?;
                    return Ok(());
                }
            };

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
退款商品類型：加班台時數卡
申請人 <@{}>
案號 {}
請說明您希望退款的理由。
=============在這則訊息以下開始處理退款=============
",
                author.id.get(),
                case_number
            );

            refund
                .send_message(
                    &ctx.serenity_context().http,
                    CreateMessage::new().content(content),
                )
                .await?;
        }
    }

    Ok(())
}
