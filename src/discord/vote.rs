use crate::database::config::Config;
use crate::database::get_connection;
use crate::{config::CONFIG, error::ServerError};
use chrono::{FixedOffset, TimeZone};
use core::panic;
use itertools::Itertools;
use poise;
use serenity::all::{ChannelId, EditMessage, ReactionType, UserId};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, OnceCell};

// 將原本的 fetch_vote_channel_and_msg 改寫為更安全的讀取函數
fn read_vote_config() -> Result<(u64, Option<u64>), ServerError> {
    let mut connection = get_connection()?;

    // [關鍵修改] 設定 10000ms (10秒) 的等待時間
    connection.busy_timeout(Duration::from_millis(10000))?;

    let transaction = connection.transaction()?; // 開啟讀取事務

    // 1. 讀取 Channel ID
    let vote_channel_id = if let Some(text) = Config::ChannelVote.get(&transaction)?
        && let Ok(channel) = text.parse::<u64>()
    {
        channel
    } else {
        return Err(ServerError::Internal(String::from(
            "Parse ChannelCoin channel id for ChannelVote failed.",
        )));
    };

    // 2. 讀取 Message ID (允許為 None)
    let vote_message_id = if let Some(text) = Config::MessageVote.get(&transaction)?
        && let Ok(message_id) = text.parse::<u64>()
    {
        Some(message_id)
    } else {
        None
    };

    // 3. 顯式提交/釋放 (雖然 Rust 會自動 drop，但在 SQLite 中顯式 commit 讀取事務是個好習慣，能確保鎖被釋放)
    // 對於唯讀事務，commit 和 rollback 效果一樣，都是結束事務
    transaction.commit()?;

    Ok((vote_channel_id, vote_message_id))
}

// 新增一個專門用來更新 Message ID 的函數
fn update_vote_message_id(new_id: u64) -> Result<(), ServerError> {
    let mut connection = get_connection()?;

    // [關鍵修改] 設定 10000ms (10秒) 的等待時間
    connection.busy_timeout(Duration::from_millis(10000))?;

    let transaction = connection.transaction()?; // 開啟寫入事務

    Config::MessageVote.set(new_id.to_string(), &transaction)?;

    transaction.commit()?; // 提交寫入
    Ok(())
}

static BALLOT: OnceCell<Arc<Mutex<Ballot>>> = OnceCell::const_new();

async fn init_ballot(ctx: super::Context<'_>) -> Result<Arc<Mutex<Ballot>>, ServerError> {
    BALLOT
        .get_or_try_init(|| async {
            let mut new_ballot = Ballot {
                deadline: None,
                options: HashMap::new(),
            };
            new_ballot.fetch(ctx).await?;
            Ok::<_, ServerError>(Arc::new(Mutex::new(new_ballot)))
        })
        .await
        .map(Arc::clone)
}

#[poise::command(slash_command)]
pub async fn vote(ctx: super::Context<'_>) -> Result<(), ServerError> {
    ctx.say("Vote Entrypoint.").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn nominate(ctx: super::Context<'_>, content: String) -> Result<(), ServerError> {
    let binding = init_ballot(ctx).await?;
    let mut ballot = binding.lock().await;
    match ballot.nominate(content, ctx.author().id) {
        Ok(()) => {
            ctx.say("提名成功").await?;
        }
        Err(e) => {
            ctx.say(format!("提名失败: {}", e)).await?;
            return Ok(());
        }
    }
    ballot.commit(ctx).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn revoke(ctx: super::Context<'_>, id: String) -> Result<(), ServerError> {
    let flag = match Flag::try_from(id.as_str()) {
        Ok(f) => f,
        Err(e) => {
            ctx.say(format!("撤回失败: {}", e)).await?;
            return Ok(());
        }
    };

    let binding = init_ballot(ctx).await?;
    let mut ballot = binding.lock().await;
    match ballot.revoke(flag, ctx.author().id) {
        Ok(()) => {
            ctx.say("撤回成功").await?;
        }
        Err(e) => {
            ctx.say(format!("撤回失败: {}", e)).await?;
            return Ok(());
        }
    };
    ballot.commit(ctx).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn deadline(
    ctx: super::Context<'_>,
    #[description = "年份 (例如 2025)"] year: i32,
    #[description = "月份 (1-12)"] month: u32,
    #[description = "日期 (1-31)"] day: u32,
) -> Result<(), ServerError> {
    // 權限檢查
    if !CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        ctx.say("权限不足").await?;
        return Ok(());
    }

    // 使用 chrono::Utc 構建日期
    // 時間固定在該日的 23:59:00 UTC+8

    let fixed_offset_8 = FixedOffset::east_opt(8 * 3600).expect("Can't offset time.");

    let datetime_opt = fixed_offset_8
        .with_ymd_and_hms(year, month, day, 23, 59, 0)
        .single();

    match datetime_opt {
        Some(dt) => {
            let ts = dt.timestamp() as u64;

            let binding = init_ballot(ctx).await?;
            let mut ballot = binding.lock().await;
            ballot.deadline = Some(ts);

            // Discord 會根據用戶所在時區顯示這個 UTC 時間戳
            ctx.say(format!("✅ 截止日期已設置為: <t:{}:f>", ts))
                .await?;

            ballot.commit(ctx).await?;
        }
        None => {
            ctx.say("❌ 無效的日期（請檢查年份、月份或該月是否有這一天）。")
                .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
// TODO: Make the vote automatically conclude the result
pub async fn conclude(ctx: super::Context<'_>) -> Result<(), ServerError> {
    if !CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        ctx.say("权限不足").await?;
        return Ok(());
    }
    let binding = init_ballot(ctx).await?;
    let mut ballot = binding.lock().await;
    ballot.deadline = None;
    ctx.say("投票已结束").await?;
    let outcome = ballot.title(ctx).await?;
    {
        ctx.say(format!("投票結果：{}", outcome)).await?;
    };
    ballot.commit(ctx).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn clear(ctx: super::Context<'_>) -> Result<(), ServerError> {
    if !CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        ctx.say("权限不足").await?;
        return Ok(());
    }

    let binding = init_ballot(ctx).await?;
    let mut ballot = binding.lock().await;
    ballot.deadline = None;
    ballot.options.clear();
    ctx.say("投票已清空").await?;
    ballot.commit(ctx).await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Ballot {
    deadline: Option<u64>,
    options: HashMap<Flag, VoteOption>,
}

impl Ballot {
    async fn fetch(&mut self, ctx: super::Context<'_>) -> Result<(), ServerError> {
        // Step 1: 讀取 (持有鎖 -> 釋放鎖)
        let (channel_vote, message_vote_opt) = read_vote_config()?;
        let channel_id = ChannelId::from(channel_vote);

        let init_process = || async {
            println!("正在初始化投票訊息..."); // Debug 用

            // Step 2: 網絡請求 (無資料庫鎖，耗時操作)
            let new_message = channel_id
                .send_message(
                    &ctx.http(),
                    serenity::all::CreateMessage::new().content("# 正在初始化投票..."),
                )
                .await?;

            println!("訊息發送成功 ID: {}，準備寫入資料庫...", new_message.id); // Debug 用

            // Step 3: 寫入 (持有鎖 -> 釋放鎖)
            // 這裡現在有了 busy_timeout，如果資料庫忙碌，它會等待而不是報錯
            match update_vote_message_id(new_message.id.get()) {
                Ok(_) => println!("資料庫寫入成功"),
                Err(e) => {
                    // 如果還是失敗，至少我們知道是在這一步
                    println!("資料庫寫入失敗: {:?}", e);
                    return Err(e);
                }
            }

            Ok::<_, ServerError>(new_message)
        };

        let message = match message_vote_opt {
            Some(message_id) => {
                match channel_id.message(&ctx.http(), message_id).await {
                    Ok(msg) => msg,
                    // Discord 找不到該訊息 -> 觸發初始化
                    Err(_) => init_process().await?,
                }
            }
            // 資料庫沒紀錄 -> 觸發初始化
            None => init_process().await?,
        };

        // 解析選項
        for options in message
            .content
            .lines()
            .skip(1)
            .filter_map(|l| VoteOption::parse(l))
        {
            self.options.insert(options.flag, options);
        }
        Ok(())
    }

    async fn commit(&self, ctx: super::Context<'_>) -> Result<(), ServerError> {
        // Step 1: Fetch the message and reactions
        // Step 2: Remove reactions that are no longer in options, and sort options based on existing reactions
        // Step 3: Add reactions that are in options but not in reactions, and sort options based on adding order
        // Step 4: Update the message content

        // 使用新的讀取函數
        let (channel_vote, message_vote_opt) = read_vote_config()?;

        // 如果要 Commit 但找不到 ID，這是不正常的 (因為 fetch 應該已經處理過了)
        let message_vote = message_vote_opt
            .ok_or_else(|| ServerError::Internal("無法提交投票：找不到投票訊息 ID".to_string()))?;

        let mut message = ChannelId::from(channel_vote)
            .message(&ctx.http(), message_vote)
            .await?;

        let mut content = Vec::new();
        let mut hashmap = self.options.clone();
        let reactions = &message.reactions;

        // add title
        content.push(self.title(ctx).await?);

        // remove reactions that are no longer in options
        for reaction in reactions {
            if let Ok(flag) = Flag::try_from(reaction.reaction_type.clone()) {
                if hashmap.contains_key(&flag) {
                    // sort options based on existing reactions
                    content.push(hashmap[&flag].to_string());
                    // keep the reaction if it's still in options
                    hashmap.remove(&flag);
                    continue;
                }
            }

            // otherwise, remove the reaction
            message
                .delete_reaction_emoji(&ctx.http(), reaction.reaction_type.clone())
                .await?;
        }

        // add reactions that are in options but not in reactions
        for (flag, option) in hashmap {
            message.react(&ctx.http(), flag.reaction()).await?;
            // sort options based on adding order
            content.push(option.to_string());
        }

        // convert content to a single string
        let content = content.iter().map(|o| o.to_string()).join("\n");

        println!("Committing ballot:\n{}", content);

        message
            .edit(&ctx.http(), EditMessage::new().content(content))
            .await?;

        Ok(())
    }

    pub fn nominate(&mut self, description: String, nominee: UserId) -> Result<(), String> {
        if self.deadline.is_none() {
            return Err("当前投票尚未开始".to_string());
        }

        let mut flags = Flag::all();
        self.options.iter().for_each(|o| {
            // remove used flags
            flags.remove(&o.0);
        });

        if let Some(next_flag) = flags.iter().next() {
            self.options.insert(
                *next_flag,
                VoteOption {
                    flag: *next_flag,
                    description,
                    nominee,
                },
            );
            Ok(())
        } else {
            Err("选项已满".to_string())
        }
    }

    pub fn revoke(&mut self, flag: Flag, user: UserId) -> Result<(), String> {
        if self.deadline.is_none() {
            return Err("当前投票尚未开始".to_string());
        }

        fn is_authorized(nominee: UserId, user: UserId) -> bool {
            CONFIG.discord.admin.contains(&user.get()) || user == nominee
        }

        if let Some(option) = self.options.get(&flag) {
            if is_authorized(option.nominee, user) {
                self.options.remove(&flag);
                Ok(())
            } else {
                Err("您没有权限".to_string())
            }
        } else {
            Err("未找到该提名".to_string())
        }
    }

    pub async fn title(&self, ctx: super::Context<'_>) -> Result<String, ServerError> {
        if let Some(deadline) = self.deadline {
            Ok(format!("当前投票截止时间: __**<t:{}:f>**__", deadline))
        } else {
            // 使用新的讀取函數
            let (channel_vote, message_vote_opt) = read_vote_config()?;

            // 處理 None 的情況
            let message_vote = match message_vote_opt {
                Some(id) => id,
                None => return Ok("# __**当前没有投票**__".to_string()),
            };

            let reactions = ChannelId::from(channel_vote)
                .message(&ctx.http(), message_vote)
                .await?
                .reactions;

            let reactions = reactions
                .iter()
                .filter_map(|r| {
                    Flag::try_from(r.reaction_type.clone())
                        .map(|f| (f, r.count))
                        .ok()
                })
                .filter(|(f, _)| self.options.contains_key(f));

            if let Some(max_count) = reactions.clone().map(|(_, c)| c).max() {
                Ok(format!(
                    "__**当前最高票{}票, 是{}**__",
                    max_count,
                    reactions
                        .filter(|(_, c)| *c == max_count)
                        .map(|(f, _)| f.str())
                        .join(", "),
                ))
            } else {
                Ok("# __**当前没有投票**__".to_string())
            }

            // match reactions.len() {
            //     0 => Ok("__**当前没有投票**__".to_string()),
            //     _ => Ok(format!(
            //         "__**当前最高票: {}，有 {} 票**__",
            //         reactions
            //             .iter()
            //             .map(|r| Flag::try_from(r.reaction_type.clone()))
            //             .filter_map(Result::ok)
            //             .map(|f| f.str())
            //             .join(", "),
            //         reactions[0].count
            //     )),
            // }
        }
    }
}

#[derive(Debug, Clone)]
struct VoteOption {
    flag: Flag,
    description: String,
    nominee: UserId,
}

impl VoteOption {
    fn to_string(&self) -> String {
        format!(
            "{}: {} (<@{}>)",
            self.flag.str(),
            self.description,
            self.nominee
        )
    }

    fn parse(text: &str) -> Option<Self> {
        if let Some((icon, rest)) = text.split_once(": ") {
            if let Some((desc, nominee)) = rest.rsplit_once(" (<@") {
                if let Some(nominee) = nominee.strip_suffix(">)") {
                    if let Some(flag) = Flag::try_from(icon).ok() {
                        if let Ok(nominee) = nominee.parse::<u64>() {
                            return Some(VoteOption {
                                flag,
                                description: desc.trim().to_string(),
                                nominee: UserId::from(nominee),
                            });
                        }
                    }
                }
            }
        }

        return None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Flag(u32);

impl Flag {
    fn id(&self) -> u32 {
        self.0
    }

    fn str(&self) -> &'static str {
        <&'static str>::from(*self)
    }

    fn reaction(&self) -> ReactionType {
        ReactionType::from(*self)
    }

    fn all() -> HashSet<Flag> {
        (0..20).map(|i| Flag(i)).collect()
    }
}

impl TryFrom<&str> for Flag {
    type Error = ServerError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "🇦🇷" => Ok(Flag(0)),
            "🇦🇺" => Ok(Flag(1)),
            "🇧🇷" => Ok(Flag(2)),
            "🇨🇦" => Ok(Flag(3)),
            "🇹🇼" => Ok(Flag(4)),
            "🇫🇷" => Ok(Flag(5)),
            "🇩🇪" => Ok(Flag(6)),
            "🇮🇳" => Ok(Flag(7)),
            "🇮🇩" => Ok(Flag(8)),
            "🇮🇹" => Ok(Flag(9)),
            "🇯🇵" => Ok(Flag(10)),
            "🇰🇷" => Ok(Flag(11)),
            "🇲🇽" => Ok(Flag(12)),
            "🇷🇺" => Ok(Flag(13)),
            "🇸🇦" => Ok(Flag(14)),
            "🇿🇦" => Ok(Flag(15)),
            "🇹🇷" => Ok(Flag(16)),
            "🇬🇧" => Ok(Flag(17)),
            "🇺🇸" => Ok(Flag(18)),
            "🇪🇺" => Ok(Flag(19)),
            _ => Err(ServerError::Internal("Invalid flag emoji".to_string())),
        }
    }
}

impl TryFrom<ReactionType> for Flag {
    type Error = ServerError;

    fn try_from(value: ReactionType) -> Result<Self, Self::Error> {
        match value {
            ReactionType::Unicode(s) => Flag::try_from(s.as_str()),
            _ => Err(ServerError::Internal("Invalid reaction type".to_string())),
        }
    }
}

impl From<Flag> for &'static str {
    fn from(flag: Flag) -> Self {
        match flag.0 {
            0 => "🇦🇷",
            1 => "🇦🇺",
            2 => "🇧🇷",
            3 => "🇨🇦",
            4 => "🇹🇼",
            5 => "🇫🇷",
            6 => "🇩🇪",
            7 => "🇮🇳",
            8 => "🇮🇩",
            9 => "🇮🇹",
            10 => "🇯🇵",
            11 => "🇰🇷",
            12 => "🇲🇽",
            13 => "🇷🇺",
            14 => "🇸🇦",
            15 => "🇿🇦",
            16 => "🇹🇷",
            17 => "🇬🇧",
            18 => "🇺🇸",
            19 => "🇪🇺",
            _ => panic!("Invalid flag id"),
        }
    }
}

impl From<Flag> for String {
    fn from(flag: Flag) -> Self {
        String::from(<&'static str>::from(flag))
    }
}

impl From<Flag> for ReactionType {
    fn from(flag: Flag) -> Self {
        ReactionType::Unicode(String::from(<&'static str>::from(flag)))
    }
}

impl From<Flag> for u32 {
    fn from(flag: Flag) -> Self {
        flag.id()
    }
}
