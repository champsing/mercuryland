use crate::{config::CONFIG, error::ServerError};
use chrono::DateTime;
use core::panic;
use itertools::Itertools;
use poise;
use serenity::all::{ChannelId, EditMessage, ReactionType, UserId};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

const MESSAGE_ID: u64 = 1415245626983059456;
const CHANNEL_ID: u64 = 1414180925591392316;

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
pub async fn deadline(ctx: super::Context<'_>, deadline: u64) -> Result<(), ServerError> {
    if !CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        ctx.say("权限不足").await?;
        return Ok(());
    }

    let binding = init_ballot(ctx).await?;
    let mut ballot = binding.lock().await;
    ballot.deadline = Some(deadline);
    ctx.say(format!("截止时间设置为: <t:{}:f>", deadline))
        .await?;
    ballot.commit(ctx).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn conclude(ctx: super::Context<'_>) -> Result<(), ServerError> {
    if !CONFIG.discord.admin.contains(&ctx.author().id.get()) {
        ctx.say("权限不足").await?;
        return Ok(());
    }

    let binding = init_ballot(ctx).await?;
    let mut ballot = binding.lock().await;
    ballot.deadline = None;
    ctx.say("投票已结束").await?;
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
        let message = ChannelId::from(CHANNEL_ID)
            .message(&ctx.http(), MESSAGE_ID)
            .await?;

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

        let mut message = ChannelId::from(CHANNEL_ID)
            .message(&ctx.http(), MESSAGE_ID)
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
            Ok(format!("__**当前投票截止时间: <t:{}:f>**__", deadline))
        } else {
            let mut reactions = ChannelId::from(CHANNEL_ID)
                .message(&ctx.http(), MESSAGE_ID)
                .await?
                .reactions;
            reactions.sort_by_key(|r| u64::MAX - r.count);

            if let Some(reaction) = reactions.first()
                && let Ok(flag) = Flag::try_from(reaction.reaction_type.clone())
                && self.options.len() > 0
            {
                Ok(format!(
                    "__**当前最高票: {}，有 {} 票**__",
                    flag.str(),
                    reaction.count
                ))
            } else {
                Ok("__**当前没有投票**__".to_string())
            }
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
