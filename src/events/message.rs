use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

/// Called when the bot gets a direct message from a [`User`].
#[tracing::instrument(skip_all, fields(message_id = %message.id), err(Debug))]
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
            request_name_confirm(context, message, bot_data).await
        },
        // Ignore everything else
        _ => Ok(()),
    }?;

    Ok(())
}

#[tracing::instrument(skip_all, err(Debug))]
async fn request_name_confirm(
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
                .embed(embeds::registration::name_confirm(&name_message.content))
                .components(components::confirm_name_buttons())
                .reference_message(name_message)
        })
        .await?;

    Ok(())
}
