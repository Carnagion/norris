use poise::serenity_prelude as serenity;

use serenity::{colours::branding::BLURPLE, *};

use crate::{events::INSTRUCTIONS_CONTINUE, prelude::*};

pub async fn messaged(context: &Context, message: &Message, bot_data: &BotData) -> BotResult<()> {
    // Try to get the user's registration status
    let registration_status = sqlx::query_file!(
        "queries/select-registration-status.sql",
        message.author.id.0,
    )
    .try_map(|row| RegistrationStatus::from_columns(row.status, row.name))
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
    sqlx::query_file!(
        "queries/update-registration-state.sql",
        RegistrationStatus::NameEntered("".to_owned()).to_string(),
        name_message.content,
        name_message.author.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask user to confirm their name
    name_message
        .channel_id
        .send_message(&context.http, |message| {
            message.embed(confirm_name_embed(&name_message.content))
        })
        .await?;

    Ok(())
}

fn confirm_name_embed(name: &str) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("You entered the name `{}`. Is that correct?", name))
    }
}
