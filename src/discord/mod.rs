mod coin;
mod give;
mod help;
mod link;
mod wheel;
mod refund;

use once_cell::sync::OnceCell as OnceLock;

use crate::{config::CONFIG, error::ServerError};
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use poise::{self};
use serenity::all::{CreateMessage, Http};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

type Data = ();
type Context<'a> = poise::Context<'a, Data, ServerError>;

static HTTP: OnceLock<Arc<Http>> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub enum Receiver {
    ChannelId(u64),
    UserId(u64),
}

impl Receiver {
    pub async fn message(&self, message: CreateMessage) -> Result<(), ServerError> {
        match self {
            Receiver::ChannelId(id) => {
                let channel_id = serenity::model::id::ChannelId::from(*id);
                channel_id.send_message(HTTP.wait(), message).await?;
            }
            Receiver::UserId(id) => {
                let user_id = serenity::model::id::UserId::from(*id);
                user_id.direct_message(HTTP.wait(), message).await?;
            }
        }

        Ok(())
    }
}

// pub async fn send_text(channel_id: ChannelId, text: &String) -> Result<(), ServerError> {
//     send_message(channel_id, vec![], &json!({"content": text})).await?;
//     Ok(())
// }

pub async fn run() -> Result<(), ServerError> {
    let zh_tw = "zh-TW".to_string();

    let options = poise::FrameworkOptions {
        commands: vec![
            poise::Command {
                name: String::from("fetch_wheel"),
                description: Some(String::from(
                    "Fetch the text in the Drawn Zone of the specified wheel",
                )),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("獲取輪盤抽中區"),
                )]),
                help_text: Some(String::from(
                    "獲取輪盤抽中區",
                )),
                ..wheel::fetch_wheel()
            },
            poise::Command {
                name: String::from("coin"),
                description: Some(String::from("Query your Mercury Coin balance")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("查詢水星幣餘額"),
                )]),
                help_text: Some(String::from(
                    "查詢您的水星幣餘額。",
                )),
                ..coin::coin()
            },
            poise::Command {
                name: String::from("give"),
                description: Some(String::from("[Admin] give user coins")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("???"),
                )]),
                help_text: Some(String::from(
                    "????",
                )),
                ..give::give()
            },
            poise::Command {
                name: String::from("link"),
                description: Some(String::from(
                    "Link your Discord account to your YouTube channel record",
                )),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("連結 Discord 帳號至 YouTube 頻道，24小時內限用一次"),
                )]),
                help_text: Some(String::from(
                    "連結您的 Discord 帳號至 YouTube 頻道後，即可直接使用 </coin> 指令查詢餘額。目前同一 Discord 帳號僅可連結1個 YouTube 頻道。",
                )),
                cooldown_config: RwLock::new(
                    poise::CooldownConfig {
                        user: Some(Duration::from_secs(86400)),
                        global: None,
                        guild: None,
                        channel: None,
                        member: None,
                        __non_exhaustive: ()
                    }),
                ..link::link()
            },
            poise::Command {
                name: String::from("unlink"),
                description: Some(String::from(
                    "Unlink your Discord account and your YouTube channel record",
                )),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("斷開 YouTube 頻道與 Discord 帳號的連結"),
                )]),
                help_text: Some(String::from(
                    "將連結斷開後，使用 </coin> 查詢餘額重新需要輸入 YouTube Channel ID，直至連結新 Discord 帳號。",
                )),
                ..link::unlink()
            },
            poise::Command {
                name: String::from("help"),
                description: Some(String::from(
                    "Show help text",
                )),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("顯示指令使用教學"),
                )]),
                help_text: Some(String::from(
                    "顯示指令使用教學",
                )),
                ..help::help()
            },
            poise::Command {
                name: String::from("refund"),
                description: Some(String::from(
                    "Refund commands",
                )),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("退款相關指令"),
                )]),
                help_text: Some(String::from(
                    "退款相關指令",
                )),
                subcommands: vec![
                    poise::Command {
                        name: String::from("new"),
                        description: Some(String::from(
                            "Open a thread channel for requesting a Mercury Coin refund",
                        )),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("開啟一則討論串以申請水星幣退款"),
                        )]),
                        help_text: Some(String::from(
                            "開啟一則討論串以申請水星幣退款",
                        )),
                        ..refund::new()
                    },
                    // poise::Command {
                    //     name: String::from("refund close"),
                    //     description: Some(String::from(
                    //         "[ADMIN] Close a Mercury Coin refund request",
                    //     )),
                    //     description_localizations: HashMap::from([(
                    //         zh_tw.clone(),
                    //         String::from("（管理）將討論串案號結單"),
                    //     )]),
                    //     help_text: Some(String::from(
                    //         "（管理）將討論串案號結單",
                    //     )),
                    //     ..refund::close()
                    // },
                ],
                subcommand_required: true,
                ..refund::refund()
            },
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .options(options)
        .build();

    let mut client = ClientBuilder::new(
        &CONFIG.discord.token,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await?;

    HTTP.get_or_init(|| client.http.clone());
    client.start().await?;

    Ok(())
}
