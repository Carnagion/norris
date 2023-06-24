use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn yes_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    let user = &component_interaction.user;

    let registration = sqlx::query!(
        "select name, kind from registrations where user_id = ? limit 1",
        user.id.0,
    )
    .fetch_one(&bot_data.database_pool)
    .await?;

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
        registration.name,
        registration.kind,
    )
    .execute(&bot_data.database_pool)
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

    Ok(())
}

pub async fn no_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
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

    Ok(())
}
