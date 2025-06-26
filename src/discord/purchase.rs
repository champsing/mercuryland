use crate::coin::command::CoinCommandManager;
use crate::config::CONFIG;
use crate::database::{coin::Coin as CoinUser, get_connection};

use crate::error::ServerError;

use poise::{self, CreateReply};
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

// #[poise::command(slash_command)]
// pub async fn overtime(ctx: super::Context<'_>, hours: u32) -> Result<(), ServerError> {
//     ctx.say("Purchase Entrypoint.").await?;
//     Ok(())
// }
