use std::{
    collections::{HashMap, HashSet},
};

use crate::{
    config::CONFIG,
    error::ServerError,
};
use itertools::Itertools;
use phf::phf_map;
use poise;
use serenity::all::{ChannelId, CreateMessage, EditMessage, MessageId, ReactionType, UserId};

const MESSAGE_ID: u64 = 1414189052483207229;
const CHANNEL_ID: u64 = 1414180925591392316;

#[poise::command(slash_command)]
pub async fn vote(ctx: super::Context<'_>) -> Result<(), ServerError> {
    ctx.say("Vote Entrypoint.").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn nominate(ctx: super::Context<'_>, content: String) -> Result<(), ServerError> {
    let mut vote = Vote::new(ctx).await?;
    match vote.nominate(content, ctx.author().id) {
        Ok(id) => {
            ctx.say(format!("提名成功，您的选项编号是 {}", ICON[&id]))
                .await?;
            vote.commit(ctx).await?;
            Vote::add_react(ctx, id).await?;
        }
        Err(e) => {
            ctx.say(format!("提名失败: {}", e)).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn revoke(ctx: super::Context<'_>, id: String) -> Result<(), ServerError> {
    let mut vote = Vote::new(ctx).await?;

    fn parse_id(id: &str) -> Option<u32> {
        if let Some(d) = id.chars().next() {
            match d {
                '0'..='9' => Some(d as u32 - '0' as u32),
                'A'..='J' => Some(d as u32 - 'A' as u32 + 10),
                'a'..='j' => Some(d as u32 - 'a' as u32 + 10),
                _ => None,
            }
        } else {
            None
        }
    }

    if let Some(id) = parse_id(id.as_str()) {
        match vote.revoke(id, ctx.author().id) {
            Ok(id) => {
                ctx.say(format!("撤回成功，撤回的选项编号是 {}", ICON[&id]))
                    .await?;
                vote.commit(ctx).await?;
                Vote::del_react(ctx, id).await?;
            }
            Err(e) => {
                ctx.say(format!("撤回失败: {}", e)).await?;
            }
        }

        Ok(())
    } else {
        ctx.say("无效的选项编号").await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Vote {
    description: String,
    options: HashMap<u32, VoteOption>,
}

impl Vote {
    async fn new(ctx: super::Context<'_>) -> Result<Self, ServerError> {
        let message = ChannelId::from(CHANNEL_ID)
            .message(&ctx.http(), MESSAGE_ID)
            .await?;
        let mut lines = message.content.lines();

        let description = lines.next().unwrap_or_default().to_string();
        lines.next(); // skip empty line

        let mut options = HashMap::new();
        for line in lines {
            if let Some(option) = VoteOption::parse(line) {
                options.insert(option.id, option);
            }
        }

        Ok(Vote {
            description,
            options,
        })
    }

    async fn commit(&self, ctx: super::Context<'_>) -> Result<(), ServerError> {
        let content = format!(
            "{}\n\n{}",
            self.description,
            self.options
                .iter()
                .sorted_by_key(|o| o.0)
                .map(|o| o.1.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );

        ChannelId::from(CHANNEL_ID)
            .message(&ctx.http(), MESSAGE_ID)
            .await?
            .edit(&ctx.http(), EditMessage::new().content(content))
            .await?;

        Ok(())
    }

    pub async fn add_react(ctx: super::Context<'_>, id: u32) -> Result<(), ServerError> {
        if let Some(icon) = ICON.get(&id) {
            if let Ok(reaction) = ReactionType::try_from(icon.to_owned()) {
                ChannelId::from(CHANNEL_ID)
                    .message(&ctx.http(), MESSAGE_ID)
                    .await?
                    .react(&ctx.http(), reaction)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn del_react(ctx: super::Context<'_>, id: u32) -> Result<(), ServerError> {
        if let Some(icon) = ICON.get(&id) {
            if let Ok(reaction) = ReactionType::try_from(icon.to_owned()) {
                ChannelId::from(CHANNEL_ID)
                    .message(&ctx.http(), MESSAGE_ID)
                    .await?
                    .delete_reaction_emoji(&ctx.http(), reaction)
                    .await?;
            }
        }

        Ok(())
    }

    pub fn nominate(&mut self, description: String, nominee: UserId) -> Result<u32, String> {
        // a person can only nominate once
        // a nomination can only happen if there is space
        // if self.options.values().any(|o| o.nominee == nominee) {
        //     Err("您已提名".to_string())
        // } else
        if let Some(id) = (0..ICON.len() as u32).find(|i| !self.options.contains_key(i)) {
            self.options.insert(
                id,
                VoteOption {
                    id,
                    description,
                    nominee,
                },
            );

            Ok(id)
        } else {
            Err("选项已满".to_string())
        }
    }

    pub fn revoke(&mut self, id: u32, user: UserId) -> Result<u32, String> {
        fn is_authorized(nominee: UserId, user: UserId) -> bool {
            CONFIG.discord.admin.contains(&user.get()) || user == nominee
        }

        if let Some(option) = self.options.get(&id) {
            if is_authorized(option.nominee, user) {
                self.options.remove(&id);
                Ok(id)
            } else {
                Err("您没有权限".to_string())
            }
        } else {
            Err("未找到该提名".to_string())
        }
    }
}

#[derive(Debug, Clone)]
struct VoteOption {
    id: u32,
    description: String,
    nominee: UserId,
}

impl VoteOption {
    fn to_string(&self) -> String {
        format!(
            "{}: {} (<@{}>)",
            ICON[&self.id], self.description, self.nominee
        )
    }

    fn parse(text: &str) -> Option<Self> {
        if let Some((icon, rest)) = text.split_once(": ") {
            // println!("Parsing option: icon='{}', rest='{}'", icon, rest);
            if let Some((desc, nominee)) = rest.rsplit_once(" (<@") {
                // println!("  desc='{}', nominee='{}'", desc, nominee);
                if let Some(nominee) = nominee.strip_suffix(">)") {
                    // println!("  nominee stripped='{}'", nominee);
                    if let Some(&id) = INDEX.get(icon.trim()) {
                        // println!("  icon id={}", id);
                        if let Ok(nominee) = nominee.parse::<u64>() {
                            // println!("  nominee id={}", nominee);
                            return Some(VoteOption {
                                id,
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

static INDEX: phf::Map<&'static str, u32> = phf_map! {
    "0️⃣" => 0,
    "1️⃣" => 1,
    "2️⃣" => 2,
    "3️⃣" => 3,
    "4️⃣" => 4,
    "5️⃣" => 5,
    "6️⃣" => 6,
    "7️⃣" => 7,
    "8️⃣" => 8,
    "9️⃣" => 9,
    "🇦" => 10,
    "🇧" => 11,
    "🇨" => 12,
    "🇩" => 13,
    "🇪" => 14,
    "🇫" => 15,
    "🇬" => 16,
    "🇭" => 17,
    "🇮" => 18,
    "🇯" => 19,
    // "🇰" => 20,
    // "🇱" => 21,
    // "🇲" => 22,
    // "🇳" => 23,
    // "🇴" => 24,
    // "🇵" => 25,
    // "🇶" => 26,
    // "🇷" => 27,
    // "🇸" => 28,
    // "🇹" => 29,
    // "🇺" => 30,
    // "🇻" => 31,
    // "🇼" => 32,
    // "🇽" => 33,
    // "🇾" => 34,
    // "🇿" => 35,
};

static ICON: phf::Map<u32, &'static str> = phf_map! {
    0 => "0️⃣",
    1 => "1️⃣",
    2 => "2️⃣",
    3 => "3️⃣",
    4 => "4️⃣",
    5 => "5️⃣",
    6 => "6️⃣",
    7 => "7️⃣",
    8 => "8️⃣",
    9 => "9️⃣",
    10 => "🇦",
    11 => "🇧",
    12 => "🇨",
    13 => "🇩",
    14 => "🇪",
    15 => "🇫",
    16 => "🇬",
    17 => "🇭",
    18 => "🇮",
    19 => "🇯",
    // 20 => "🇰",
    // 21 => "🇱",
    // 22 => "🇲",
    // 23 => "🇳",
    // 24 => "🇴",
    // 25 => "🇵",
    // 26 => "🇶",
    // 27 => "🇷",
    // 28 => "🇸",
    // 29 => "🇹",
    // 30 => "🇺",
    // 31 => "🇻",
    // 32 => "🇼",
    // 33 => "🇽",
    // 34 => "🇾",
    // 35 => "🇿",
};
