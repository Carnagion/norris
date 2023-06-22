use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn guild_member_added(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
) -> BotResult<()> {
    // Add the member as unregistered to the table of registration info
    sqlx::query_file!(
        "queries/insert-new-registration.sql",
        member.user.id.0,
        RegistrationStatus::Unregistered.to_string(),
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Send instructions for registration to the user's DMs
    let instructions_sent = send_registration_instructions(context, member).await;
    match instructions_sent {
        // Ask user to check DMs
        Ok(_) => notify_instructions_sent(context, member, bot_data).await,
        // Handle failure
        Err(error) => {
            log::error!("{}", error);

            // Update their registration status to be failed
            sqlx::query_file!(
                "queries/update-registration-state.sql",
                RegistrationStatus::Failed.to_string(),
                None::<String>,
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

async fn send_registration_instructions(context: &Context, member: &Member) -> BotResult<()> {
    member
        .user
        .direct_message(&context.http, |message| {
            message
                .embed(responses::instructions_embed(member.user.id))
                .components(responses::instructions_continue_button())
        })
        .await?;
    Ok(())
}

async fn notify_instructions_sent(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
) -> BotResult<()> {
    bot_data
        .arrival_channel_id
        .send_message(&context.http, |message| {
            message
                .embed(responses::instructions_sent_embed(member.user.id))
                .components(responses::instructions_sent_button())
        })
        .await?;

    Ok(())
}

async fn notify_instructions_error(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
) -> BotResult<()> {
    bot_data
        .arrival_channel_id
        .send_message(&context.http, |message| {
            message.embed(responses::instructions_error_embed(
                member.user.id,
                bot_data.support_channel_id,
            ))
        })
        .await?;

    Ok(())
}
