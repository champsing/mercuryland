use crate::{
    config::{AUTH_CODE, AuthCode, CONFIG},
    error::ServerError,
};
use chrono::{Duration, Utc};
use poise::reply::CreateReply;
use rand::{Rng, distributions::Alphanumeric, thread_rng};

#[poise::command(slash_command)]
pub async fn auth(ctx: super::Context<'_>) -> Result<(), ServerError> {
    if !CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        ctx.send(CreateReply::default().content("您没有权限").ephemeral(true))
            .await?;
        return Ok(());
    }

    let mut code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    code.make_ascii_uppercase();

    let now = Utc::now();
    let expires_at = now + Duration::minutes(30);

    {
        let mut stored_code = AUTH_CODE
            .write()
            .map_err(|err| ServerError::Internal(format!("auth code lock poisoned: {}", err)))?;
        stored_code.retain(|entry| entry.expires_at > now);
        stored_code.push(AuthCode {
            code: code.clone(),
            expires_at,
        });
    }

    ctx.send(
        CreateReply::default()
            .content(format!("您的驗證碼為：{}", code))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
