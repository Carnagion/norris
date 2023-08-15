//! Slash commands for requesting a nickname.

use std::time::Duration;

use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

/// Request a nickname change.
#[poise::command(slash_command, guild_only)]
#[tracing::instrument(skip(context), err(Debug))]
pub async fn nickname(
    context: BotContext<'_>,
    #[description = "Your new nickname."] nickname: String,
) -> BotResult<()> {
    // Acknowledge nickname request
    context
        .send(|reply| reply.embed(embeds::nickname::acknowledge_request()))
        .await?;

    let user = context.author();
    let bot_data = context.data();
    let http = &context.serenity_context().http;

    // Fetch the name that the user is registered under
    let name = sqlx::query!(
        "select name from users
        where registered_user_id = ?
        limit 1",
        user.id.0,
    )
    .fetch_one(&bot_data.database_pool)
    .await?
    .name;

    // Fetch the user's current nickname
    let current_nickname = user.nick_in(http, context.data().guild_id).await;
    let current_nickname = current_nickname.as_deref().unwrap_or(&user.name);

    // Post an embed for approval or denial of new nickname
    let message = bot_data
        .channels
        .nickname_channel_id
        .send_message(http, |message| {
            message
                .embed(embeds::nickname::request_approval(
                    user.id,
                    &name,
                    current_nickname,
                    &nickname,
                ))
                .components(components::nickname_approval_buttons())
        })
        .await?;

    // Wait for approval or denial
    let collector = CollectComponentInteraction::new(context)
        .guild_id(bot_data.guild_id)
        .channel_id(bot_data.channels.nickname_channel_id)
        .message_id(message.id)
        .filter(|interaction| {
            interaction.data.custom_id == components::NICKNAME_APPROVE
                || interaction.data.custom_id == components::NICKNAME_DENY
        })
        .collect_limit(1)
        .timeout(Duration::from_secs(900));
    if let Some(interaction) = collector.await {
        // Find whether it was approved or denied
        let approved = match interaction.data.custom_id.as_str() {
            components::NICKNAME_APPROVE => true,
            components::NICKNAME_DENY => false,
            _ => unreachable!(), // PANICS: The filter ensures only these interactions are collected
        };

        if approved {
            // Change the user's nickname
            let member = bot_data.guild_id.member(http, user.id).await?;
            member
                .edit(http, |member| member.nickname(&nickname))
                .await?;

            // Respond with approval
            context
                .send(|reply| reply.embed(embeds::nickname::approved(&nickname)))
                .await
        } else {
            // Respond with denial
            context
                .send(|reply| reply.embed(embeds::nickname::denied(&nickname)))
                .await
        }?;
    }

    // Delete the nickname embed sent to mentors
    message.delete(context.http()).await?;

    Ok(())
}
