use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn restart(context: BotContext<'_>, mut member: Member) -> BotResult<()> {
    // Restart the user's registration process
    restart_registration(context, &mut member).await?;

    // Reply after restarting their registration
    context
        .send(|reply| reply.embed(responses::registration_restart_embed(member.user.id)))
        .await?;

    Ok(())
}

pub(super) async fn restart_registration(
    context: BotContext<'_>,
    member: &mut Member,
) -> BotResult<()> {
    let user_id = member.user.id;

    // Reset the user's registration state to started
    sqlx::query!(
        "update registrations set status = ?, name = null, kind = null where user_id = ?",
        RegistrationStatus::Started.to_string(),
        user_id.0,
    )
    .execute(&context.data().database_pool)
    .await?;

    // Delete their registered user ID
    sqlx::query!(
        "update users set registered_user_id = null where registered_user_id = ?",
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

    // Ask the user to enter their name
    member
        .user
        .direct_message(context.http(), |message| {
            message.embed(responses::request_name_embed())
        })
        .await?;

    // Log the registration restart
    context
        .data()
        .channels
        .log_channel_id
        .send_message(context.http(), |message| {
            message.embed(responses::registration_restart_log_embed(user_id))
        })
        .await?;

    Ok(())
}
