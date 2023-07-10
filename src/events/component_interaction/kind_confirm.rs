use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

pub async fn yes_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    name: String,
    kind: VerifiedUserKind,
) -> BotResult<()> {
    let user = &component_interaction.user;

    // Confirm the user as verified, but not registered yet
    sqlx::query!(
        "update registrations
        set status = ?, name = null, kind = null
        where user_id = ?",
        RegistrationStatus::Verified.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Link the user's account to the verified user
    sqlx::query!(
        "update users
        set registered_user_id = ?
        where name = ? and kind = ? and registered_user_id is null
        limit 1", // NOTE: Only one user should be consdiered verified
        user.id.0,
        name,
        kind.to_string(),
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Inform the user of verification and extra questions
    component_interaction
        .create_followup_message(&context.http, |message| {
            message
                .embed(embeds::registration::verified())
                .components(components::verified_continue_button())
        })
        .await?;

    // Log the verification
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::logs::verified(user.id, kind))
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
        "update registrations
        set status = ?, name = null
        where user_id = ?",
        RegistrationStatus::Failed.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to seek assistance
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(embeds::registration::kind_error(
                bot_data.channels.support_channel_id,
            ))
        })
        .await?;

    // Alert the mentors about incorrect kind
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message
                .content(format!("<@&{}>", bot_data.roles.hierarchy.mentor_role_id))
                .embed(embeds::logs::kind_error(
                    user.id,
                    kind,
                    bot_data.channels.support_channel_id,
                ))
        })
        .await?;

    Ok(())
}
