use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn messaged(context: &Context, message: &Message, bot_data: &BotData) -> BotResult<()> {
    // Try to get the user's registration status
    let registration_status = sqlx::query!(
        "select status, name, kind from registrations where user_id = ?",
        message.author.id.0,
    )
    .try_map(|row| RegistrationStatus::from_columns(row.status, row.name, row.kind))
    .fetch_optional(&bot_data.database_pool)
    .await?;

    match registration_status {
        // User has entered their name and it needs to be confirmed
        Some(RegistrationStatus::Started) if !message.content.is_empty() => {
            request_confirm_name(context, message, bot_data).await
        },
        // Ignore everything else
        _ => Ok(()),
    }?;

    Ok(())
}

async fn request_confirm_name(
    context: &Context,
    name_message: &Message,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration status to name entered
    sqlx::query!(
        "update registrations set status = ?, name = ? where user_id = ?",
        RegistrationStatus::NameEntered("".into()).to_string(),
        name_message.content,
        name_message.author.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask user to confirm their name
    name_message
        .channel_id
        .send_message(&context.http, |message| {
            message
                .embed(responses::confirm_name_embed(&name_message.content))
                .components(responses::confirm_name_buttons())
                .reference_message(name_message)
        })
        .await?;

    Ok(())
}