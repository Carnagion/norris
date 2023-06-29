use std::time::Duration;

use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

#[poise::command(slash_command, guild_only)]
pub async fn nickname(context: BotContext<'_>, nickname: String) -> BotResult<()> {
    // Acknowledge nickname request
    context
        .send(|reply| reply.embed(responses::nickname_acknowledge_embed()))
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
                .embed(responses::nickname_request_embed(
                    user.id,
                    &name,
                    current_nickname,
                    &nickname,
                ))
                .components(responses::nickname_approval_buttons())
        })
        .await?;

    // Wait for approval or denial
    let collector = CollectComponentInteraction::new(context)
        .guild_id(bot_data.guild_id)
        .channel_id(bot_data.channels.nickname_channel_id)
        .message_id(message.id)
        .filter(|interaction| {
            interaction.data.custom_id == responses::NICKNAME_APPROVE
                || interaction.data.custom_id == responses::NICKNAME_DENY
        })
        .collect_limit(1)
        .timeout(Duration::from_secs(600));
    if let Some(interaction) = collector.await {
        // Find whether it was approved or denied
        let approved = match interaction.data.custom_id.as_str() {
            responses::NICKNAME_APPROVE => true,
            responses::NICKNAME_DENY => false,
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
                .send(|reply| reply.embed(responses::nickname_approved_embed(&nickname)))
                .await
        } else {
            // Respond with denial
            context
                .send(|reply| reply.embed(responses::nickname_denied_embed(&nickname)))
                .await
        }?;
    }

    Ok(())
}
