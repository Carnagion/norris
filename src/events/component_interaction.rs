use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn message_component_interacted(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    match component_interaction.data.custom_id.as_str() {
        responses::INSTRUCTIONS_CONTINUE => {
            instructions_continue_clicked(context, &component_interaction.user, bot_data).await
        },
        _ => Ok(()),
    }
}

async fn instructions_continue_clicked(
    context: &Context,
    user: &User,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration state to started
    sqlx::query!(
        "update registrations set status = ? where user_id = ?",
        RegistrationStatus::Started.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to enter their name
    user.direct_message(&context.http, |message| {
        message.embed(responses::request_name_embed())
    })
    .await?;

    Ok(())
}
