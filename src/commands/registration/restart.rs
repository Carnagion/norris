use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{events, prelude::*};

/// Restart a user's registration.
#[poise::command(slash_command, guild_only, required_permissions = "KICK_MEMBERS")]
#[tracing::instrument(skip_all, fields(member_id = %member.user.id), err(Debug))]
pub async fn restart(
    context: BotContext<'_>,
    #[description = "Whose registration to restart."] mut member: Member,
) -> BotResult<()> {
    // Defer reply to give time for database queries
    context.defer().await?;

    // Restart the user's registration process
    restart_registration(context, &mut member).await?;

    // Reply after restarting their registration
    context
        .send(|reply| reply.embed(embeds::registration::restart(member.user.id)))
        .await?;

    Ok(())
}

#[tracing::instrument(skip_all, fields(member_id = %member.user.id), err(Debug))]
pub(super) async fn restart_registration(
    context: BotContext<'_>,
    member: &mut Member,
) -> BotResult<()> {
    let user_id = member.user.id;

    // Reset the user's registration state to unregistered
    sqlx::query!(
        "insert into registrations
        value (?, ?, null, null)
        on duplicate key update
        status = ?, name = null, kind = null",
        user_id.0,
        RegistrationStatus::Unregistered.to_string(),
        RegistrationStatus::Unregistered.to_string(),
    )
    .execute(&context.data().database_pool)
    .await?;

    // Delete their registered user ID
    sqlx::query!(
        "update users
        set registered_user_id = null
        where registered_user_id = ?",
        user_id.0,
    )
    .execute(&context.data().database_pool)
    .await?;

    // Remove all roles that need registration from the member
    member
        .remove_roles(
            context.http(),
            &context.data().roles.roles_needing_registration(),
        )
        .await?;

    // Log the registration restart
    context
        .data()
        .channels
        .log_channel_id
        .send_message(context.http(), |message| {
            message.embed(embeds::logs::registration_restarted(user_id))
        })
        .await?;

    // Ask the user to start registration again
    events::try_send_instructions(
        context.serenity_context(),
        member,
        context.data(),
        |message| {
            message
                .embed(embeds::registration::instructions(user_id))
                .components(components::instructions_continue_button())
        },
    )
    .await
}
