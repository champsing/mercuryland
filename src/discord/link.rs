use crate::{coin::youtube::Coin, database::{self, verify::Verify as VerifyFlow}, error::ServerError};
use chrono::Utc;
use poise;

#[poise::command(slash_command)]
pub async fn link(
    ctx: super::Context<'_>,
    #[description = "your verification code"] code: String,
) -> Result<(), ServerError> {
    // ctx.say("您正在執行的操作是授權我們存取您的 Google 帳號以讀取您帳號旗下的 YouTube 頻道，用於將您的 Discord 帳號與 YouTube 頻道建立內部資料庫連結。").await?;

    // Here should be the database connection to store the channel id
    let _ = {
        let mut connection = database::get_connection()?;
        let transaction = connection.transaction()?;
        
        let yotube_channel_id = match VerifyFlow::by_code(code, &transaction)? {
            Some(v) => v.yt_ch_id,
            None => return Err(String::from("驗證碼錯誤或不存在").into()),
        };

        let _ = match Coin::by_id(yotube_channel_id, &transaction)? {
            Some(mut r) => {
                r.discord_id = ctx.author().id.to_string();
                r.updated_at = Utc::now();
                r.update(&transaction)?;
                transaction.commit()?;
            }
            None => return Err(String::from("YouTube 頻道不存在，請先使用 </coin:1322897222991351848> 或在直播聊天室留言以建立您的 YouTube 頻道記錄。").into()),
        };
        
    };
    
    ctx.say(format!("{}", "您已成功連結您的帳號至 YouTube 頻道")).await?;
    Ok(())
}
