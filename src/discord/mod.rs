mod coin;
mod give;
mod help;
mod link;
mod purchase;
mod refund;
mod vote;
mod wheel;

use once_cell::sync::OnceCell as OnceLock;

use crate::{config::CONFIG, error::ServerError};
use poise::serenity_prelude::CommandOptionType;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use poise::{self};
use serenity::all::{CreateCommandOption, CreateMessage, Http, Message};
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
    pub async fn message(&self, message: CreateMessage) -> Result<Message, ServerError> {
        Ok(match self {
            Receiver::ChannelId(id) => {
                let channel_id = serenity::model::id::ChannelId::from(*id);
                channel_id.send_message(HTTP.wait(), message).await?
            }
            Receiver::UserId(id) => {
                let user_id = serenity::model::id::UserId::from(*id);
                user_id.direct_message(HTTP.wait(), message).await?
            }
        })
    }
}

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
                help_text: Some(String::from("獲取輪盤抽中區")),
                parameters: vec![poise::CommandParameter {
                    name: String::from("wheel_id"),
                    name_localizations: HashMap::new(),
                    description: Some(String::from("The id of wheel session")),
                    description_localizations: HashMap::from([(
                        zh_tw.clone(),
                        String::from("輪盤連線階段的 ID"),
                    )]),
                    required: true,
                    channel_types: None,
                    choices: vec![],
                    type_setter: Some(|b: CreateCommandOption| b.kind(CommandOptionType::String)),
                    __non_exhaustive: (),
                    autocomplete_callback: None,
                }],
                ..wheel::fetch_wheel()
            },
            poise::Command {
                name: String::from("coin"),
                description: Some(String::from("Query your Mercury Coin balance")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("查詢水星幣餘額"),
                )]),
                help_text: Some(String::from("查詢您的水星幣餘額。")),
                ..coin::coin()
            },
            poise::Command {
                name: String::from("give"),
                description: Some(String::from("[Admin] give user coins")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("（管理）給予使用者水星幣。可支援正或負數。"),
                )]),
                help_text: Some(String::from("（管理）給予使用者水星幣。可支援正或負數。")),
                ..give::give()
            },
            poise::Command {
                name: String::from("link"),
                description: Some(String::from(
                    "Link your Discord account to your YouTube channel record",
                )),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("連結 Discord 帳號至 YouTube 頻道， 2 小時內限用一次"),
                )]),
                help_text: Some(String::from(format!(
                    "連結您的 Discord 帳號至 YouTube 頻道後，即可直接使用 {} 指令查詢餘額。目前同一 Discord 帳號僅可連結1個 YouTube 頻道。",
                    CONFIG.slash_command_strings.coin
                ))),
                cooldown_config: RwLock::new(poise::CooldownConfig {
                    user: Some(Duration::from_secs(60 * 60 * 2)),
                    global: None,
                    guild: None,
                    channel: None,
                    member: None,
                    __non_exhaustive: (),
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
                help_text: Some(String::from(format!(
                    "將連結斷開後，使用 {} 查詢餘額重新需要輸入 YouTube Channel ID，直至連結新 Discord 帳號。",
                    CONFIG.slash_command_strings.coin
                ))),
                ..link::unlink()
            },
            poise::Command {
                name: String::from("help"),
                description: Some(String::from("Show help text")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("顯示指令使用教學"),
                )]),
                help_text: Some(String::from("顯示指令使用教學")),
                ..help::help()
            },
            poise::Command {
                name: String::from("refund"),
                description: Some(String::from("Refund commands")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("退款相關指令"),
                )]),
                help_text: Some(String::from("退款相關指令")),
                subcommands: vec![
                    poise::Command {
                        name: String::from("close"),
                        description: Some(String::from(
                            "[ADMIN] Close a Mercury Coin refund request",
                        )),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("（管理）將討論串案號結單"),
                        )]),
                        help_text: Some(String::from("（管理）將討論串案號結單")),
                        ..refund::close()
                    },
                    poise::Command {
                        name: String::from("reopen"),
                        description: Some(String::from(
                            "[ADMIN] Reopen a Mercury Coin refund request",
                        )),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("（管理）將討論串案號重新開啟"),
                        )]),
                        help_text: Some(String::from("（管理）將討論串案號重新開啟")),
                        ..refund::reopen()
                    },
                ],
                subcommand_required: true,
                ..refund::refund()
            },
            poise::Command {
                name: String::from("purchase"),
                description: Some(String::from("Purchase a product with Mercury Coins")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("使用水星幣購買商品"),
                )]),
                help_text: Some(String::from("購買相關指令")),
                subcommands: vec![
                    poise::Command {
                        name: String::from("booster"),
                        description: Some(String::from(
                            "Purchase a booster to amplify the penalty possibility on the wheel.",
                        )),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("購買懲罰加倍卡"),
                        )]),
                        help_text: Some(String::from(
                            "購買懲罰加倍卡，使用後可將懲罰在轉盤上的機率加倍。",
                        )),
                        ..purchase::booster()
                    },
                    poise::Command {
                        name: String::from("overtime"),
                        description: Some(String::from(
                            "Purchase an overtime to force 20 stream overtime.",
                        )),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("購買加班台時數"),
                        )]),
                        help_text: Some(String::from(
                            "購買加班台時數卡，可以增加惡靈剩餘的加班台時數。",
                        )),
                        ..purchase::overtime()
                    },
                ],
                subcommand_required: true,
                ..purchase::purchase()
            },
            poise::Command {
                name: String::from("vote"),
                description: Some(String::from("Vote a game to be played in the next stream")),
                description_localizations: HashMap::from([(
                    zh_tw.clone(),
                    String::from("投票下次直播要玩的遊戲"),
                )]),
                help_text: Some(String::from("投票相關指令")),
                subcommands: vec![
                    poise::Command {
                        name: String::from("nominate"),
                        description: Some(String::from("Nominate a option.")),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("提名一個選項"),
                        )]),
                        help_text: Some(String::from("提名一個選項，讓其他人投票。")),
                        ..vote::nominate()
                    },
                    poise::Command {
                        name: String::from("revoke"),
                        description: Some(String::from("Revoke a nomination.")),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("撤回一個提名"),
                        )]),
                        help_text: Some(String::from("撤回一個提名。")),
                        ..vote::revoke()
                    },
                    poise::Command {
                        name: String::from("deadline"),
                        description: Some(String::from("Set the deadline for the voting.")),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("設置投票截止時間"),
                        )]),
                        help_text: Some(String::from("設置投票截止時間。")),
                        ..vote::deadline()
                    },
                    poise::Command {
                        name: String::from("conclude"),
                        description: Some(String::from("Count the votes and show the result.")),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("計算投票結果"),
                        )]),
                        help_text: Some(String::from("計算投票結果並顯示。")),
                        ..vote::conclude()
                    },
                    poise::Command {
                        name: String::from("clear"),
                        description: Some(String::from("Clear the voting options.")),
                        description_localizations: HashMap::from([(
                            zh_tw.clone(),
                            String::from("清空投票選項"),
                        )]),
                        help_text: Some(String::from("清空投票選項。")),
                        ..vote::clear()
                    },
                ],
                subcommand_required: true,
                ..vote::vote()
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
