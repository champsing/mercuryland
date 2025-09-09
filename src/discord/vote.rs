use std::collections::HashMap;

use crate::{config::CONFIG, error::ServerError};
use itertools::Itertools;
use phf::phf_map;
use poise;
use serenity::all::{ChannelId, EditMessage, ReactionType, UserId};

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

#[poise::command(slash_command)]
pub async fn count(ctx: super::Context<'_>) -> Result<(), ServerError> {
    let vote = Vote::new(ctx).await?;
    vote.count(ctx).await?;
    Ok(())
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

        let _ = lines.next().unwrap_or_default().to_string();
        lines.next(); // skip empty line

        let mut options = HashMap::new();
        for line in lines {
            if let Some(option) = VoteOption::parse(line) {
                options.insert(option.id, option);
            }
        }

        let description =
            "这里是水星议会的投票大厅！水星公民可以民主的决定水星神的下一场直播内容！".to_string();

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
        if let Some(next_id) = (0..ICON.len() as u32).find(|i| !self.options.contains_key(i)) {
            self.options.insert(
                next_id,
                VoteOption {
                    id: next_id,
                    description,
                    nominee,
                },
            );

            Ok(next_id)
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

    pub async fn count(&self, ctx: super::Context<'_>) -> Result<(), ServerError> {
        let mut reactions = ChannelId::from(CHANNEL_ID)
            .message(&ctx.http(), MESSAGE_ID)
            .await?
            .reactions;
        reactions.sort_by_key(|r| -(r.count as i64));
        if let Some(reaction) = reactions.first() {
            ctx.say(format!(
                "{} 是最高票，有{}票",
                reaction.reaction_type, reaction.count
            ))
            .await?;
        } else {
            ctx.say("当前没有投票").await?;
        }
        Ok(())
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
            if let Some((desc, nominee)) = rest.rsplit_once(" (<@") {
                if let Some(nominee) = nominee.strip_suffix(">)") {
                    if let Some(&id) = INDEX.get(icon.trim()) {
                        if let Ok(nominee) = nominee.parse::<u64>() {
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
    "🇦🇷" => 0,   // 阿根廷
    "🇦🇺" => 1,   // 澳大利亚
    "🇧🇷" => 2,   // 巴西
    "🇨🇦" => 3,   // 加拿大
    "🇹🇼" => 4,   // 中国
    "🇫🇷" => 5,   // 法国
    "🇩🇪" => 6,   // 德国
    "🇮🇳" => 7,   // 印度
    "🇮🇩" => 8,   // 印度尼西亚
    "🇮🇹" => 9,   // 意大利
    "🇯🇵" => 10,  // 日本
    "🇰🇷" => 11,  // 韩国
    "🇲🇽" => 12,  // 墨西哥
    "🇷🇺" => 13,  // 俄罗斯
    "🇸🇦" => 14,  // 沙特阿拉伯
    "🇿🇦" => 15,  // 南非
    "🇹🇷" => 16,  // 土耳其
    "🇬🇧" => 17,  // 英国
    "🇺🇸" => 18,  // 美国
    "🇪🇺" => 19,  // 欧盟
};

static ICON: phf::Map<u32, &'static str> = phf_map! {
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
};
