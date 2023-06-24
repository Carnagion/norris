use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn yes_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    name: String,
    kind: VerifiedUserKind,
) -> BotResult<()> {
    let user = &component_interaction.user;

    // Confirm the user as registered
    sqlx::query!(
        "update registrations set status = ?, name = null, kind = null where user_id = ?",
        RegistrationStatus::Registered.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Link the user's account to the verified user
    sqlx::query!(
        "update users set registered_user_id = ? where name = ? and kind = ? limit 1", // NOTE: Only one user should be registered
        user.id.0,
        name,
        kind.to_string(),
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Give the user the appropriate role
    bot_data
        .guild_id
        .member(&context.http, user.id)
        .await?
        .add_role(&context.http, bot_data.roles.hierarchy.role(kind))
        .await?;

    // Inform the user of completion and extra optional questions
    component_interaction
        .create_followup_message(&context.http, |message| {
            message
                .embed(responses::registered_continue_embed(
                    bot_data.channels.chat_channel_id,
                ))
                .components(responses::registered_continue_button())
        })
        .await?;

    // Log the completion of registration
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(responses::registered_log_emed(user.id, kind))
        })
        .await?;

    Ok(())
}

pub async fn no_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    kind: VerifiedUserKind,
) -> BotResult<()> {
    let user = &component_interaction.user;

    // Set the user's registration state to failed
    sqlx::query!(
        "update registrations set status = ?, name = null where user_id = ?",
        RegistrationStatus::Failed.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to seek assistance
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::kind_error_embed(
                bot_data.channels.support_channel_id,
            ))
        })
        .await?;

    // Alert the mentors about incorrect kind
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(responses::kind_error_log_embed(
                user.id,
                kind,
                bot_data.channels.support_channel_id,
                bot_data.roles.hierarchy.mentor_role_id,
            ))
        })
        .await?;

    Ok(())
}
