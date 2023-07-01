use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

pub async fn continue_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration state to started
    sqlx::query!(
        "update registrations
        set status = ?
        where user_id = ?",
        RegistrationStatus::Started.to_string(),
        component_interaction.user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to enter their name
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(embeds::registration::name_request())
        })
        .await?;

    // Log the start of registration
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::logs::registration_started(
                component_interaction.user.id,
            ))
        })
        .await?;

    Ok(())
}
