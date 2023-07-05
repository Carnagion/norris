use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

/// Called when a [`Member`] joins the [`Guild`].
pub async fn guild_member_added(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
) -> BotResult<()> {
    // Add the member as unregistered to the table of registration info
    sqlx::query!(
        "insert into registrations
        value (?, ?, null, null)",
        member.user.id.0,
        RegistrationStatus::Unregistered.to_string(),
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Log the user's entry
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::logs::user_joined(member.user.id))
        })
        .await?;

    // Try to send instructions for registration to the user and notify them
    try_send_instructions(context, member, bot_data, |message| {
        message
            .embed(embeds::registration::instructions(member.user.id))
            .components(components::instructions_continue_button())
    })
    .await
}

pub(crate) async fn try_send_instructions<'a>(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
    message: impl for<'b> FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
) -> BotResult<()> {
    // Send a DM to the user
    let instructions_sent = member.user.direct_message(&context.http, message).await;

    match instructions_sent {
        // Ask user to check DMs
        Ok(_) => notify_instructions_sent(context, member, bot_data).await,
        // Handle failure
        Err(error) => {
            log::error!("{}", error);

            // Update their registration status to be failed
            sqlx::query!(
                "update registrations
                set status = ?, name = null
                where user_id = ?",
                RegistrationStatus::Failed.to_string(),
                member.user.id.0,
            )
            .execute(&bot_data.database_pool)
            .await?;

            // Notify user of error
            notify_instructions_error(context, member, bot_data).await
        },
    }?;

    Ok(())
}

async fn notify_instructions_sent(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
) -> BotResult<()> {
    // Inform user of DM sent
    bot_data
        .channels
        .arrival_channel_id
        .send_message(&context.http, |message| {
            message
                .embed(embeds::registration::instructions_sent(member.user.id))
                .components(components::instructions_sent_button())
        })
        .await?;

    Ok(())
}

async fn notify_instructions_error(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
) -> BotResult<()> {
    // Ask the user to seek assistance
    bot_data
        .channels
        .arrival_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::registration::instructions_error(
                member.user.id,
                bot_data.channels.support_channel_id,
            ))
        })
        .await?;

    // Alert mentors of error
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::logs::dm_error(
                member.user.id,
                bot_data.channels.support_channel_id,
                bot_data.roles.hierarchy.mentor_role_id,
            ))
        })
        .await?;

    Ok(())
}
