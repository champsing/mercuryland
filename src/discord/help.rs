use crate::error::ServerError;

// #[derive(poise::ChoiceParameter, PartialEq, Debug)]
// pub enum CommandQueried {
//     #[name = "fetch_wheel"]
//     FetchWheel,
//     #[name = "coin"]
//     Coin,
//     #[name = "link"]
//     Link,
//     #[name = "unlink"]
//     Unlink,
//     #[name = "help"]
//     Help
// }

#[poise::command(slash_command)]
pub async fn help(
    ctx: super::Context<'_>,
    #[description = "The command you want to show help"]
    #[description_localized("zh-TW", "要顯示的指令")] command: Option<String>,
) -> Result<(), ServerError> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "請輸入您想查詢的指令（如：link、coin，不包含斜線）",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}