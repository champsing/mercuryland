use crate::{
    database::{self, config::Config},
    error::ServerError,
};
use poise::serenity_prelude::*;
use poise;
use serenity::all::EventHandler;

#[poise::command(slash_command)]
pub async fn anonymous(_ctx: super::Context<'_>) -> Result<(), ServerError> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn create(ctx: super::Context<'_>) -> Result<(), ServerError> {
    let channel_id = ctx.channel_id().get();

    let button = CreateButton::new("anonymous_button")
        .label("匿名發言")
        .style(ButtonStyle::Primary);

    let components = CreateActionRow::Buttons(vec![button]);

    let message = CreateMessage::new()
        .content("點擊按鈕進行匿名發言")
        .components(vec![components]);

    let channel = ChannelId::new(channel_id);
    channel.send_message(&ctx.http(), message).await?;

    ctx.send(poise::CreateReply::default().content("匿名頻道已設定").ephemeral(true)).await?;
    Ok(())
}

pub async fn handle_component(ctx: serenity::all::Context, component: serenity::all::ComponentInteraction) {
    let modal = serenity::all::CreateModal::new("anonymous_modal", "匿名發言")
        .components(vec![serenity::all::CreateActionRow::InputText(serenity::all::CreateInputText::new(serenity::all::InputTextStyle::Paragraph, "內容", "content").max_length(2000))]);
    let _ = component.create_response(&ctx.http, serenity::all::CreateInteractionResponse::Modal(modal)).await;
}

pub async fn handle_modal(ctx: serenity::all::Context, modal: serenity::all::ModalInteraction) {
    let content = if let serenity::all::ActionRowComponent::InputText(ref input_text) = modal.data.components[0].components[0] {
        input_text.value.clone().unwrap_or_default()
    } else {
        String::new()
    };
    let channel_id = modal.channel_id;
    let button = serenity::all::CreateButton::new("anonymous_button")
        .label("匿名發言")
        .style(serenity::all::ButtonStyle::Primary);
    let components = serenity::all::CreateActionRow::Buttons(vec![button]);
    let message = serenity::all::CreateMessage::new()
        .content(&content)
        .components(vec![components]);
    let _ = channel_id.send_message(&ctx.http, message).await;
    let _ = modal.create_response(&ctx.http, serenity::all::CreateInteractionResponse::Message(serenity::all::CreateInteractionResponseMessage::new().content("發送成功").ephemeral(true))).await;
}

pub struct AnonymousEventHandler;

#[serenity::async_trait]
impl EventHandler for AnonymousEventHandler {
    async fn interaction_create(&self, ctx: serenity::all::Context, interaction: serenity::all::Interaction) {
        if interaction.kind() == serenity::all::InteractionType::Component {
            let component = interaction.message_component().unwrap();
            if component.data.custom_id == "anonymous_button" {
                handle_component(ctx, component).await;
            }
        } else if interaction.kind() == serenity::all::InteractionType::Modal {
            let modal = interaction.modal_submit().unwrap();
            if modal.data.custom_id == "anonymous_modal" {
                handle_modal(ctx, modal).await;
            }
        }
    }
}