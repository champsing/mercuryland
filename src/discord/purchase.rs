use crate::coin::command::CoinCommandManager;
use crate::database::{coin::Coin as CoinUser, get_connection};

use crate::error::ServerError;

use poise::{self, CreateReply};
use serenity::futures::lock::Mutex;
use std::sync::LazyLock;

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

    let find_coin_user: Result<Option<String>, ServerError> = {
        let mut connection = get_connection()?;
        let transaction = connection.transaction()?;
        let user_id = match CoinUser::by_discord(author.id.get().to_string(), &transaction)? {
            Some(u) => Some(u.id),
            None => None,
        };
        Ok(user_id)
    };

    //get Coin user by discord id
    let author_youtube_channel_id = {
        let user_id = match find_coin_user? {
            Some(id) => id,
            None => {
                ctx.send(
                CreateReply::default()
                    .content("查無資料，請先使用 </link:1327669445283414117> 將 Discord 帳號關聯到您的 YouTube 頻道。")
                    .ephemeral(true),
                ).await?;
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
    Ok(())
}

// #[poise::command(slash_command)]
// pub async fn overtime(ctx: super::Context<'_>, hours: u32) -> Result<(), ServerError> {
//     ctx.say("Purchase Entrypoint.").await?;
//     Ok(())
// }
