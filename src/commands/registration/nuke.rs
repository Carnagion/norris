use poise::{
    futures_util::{future, TryStreamExt},
    serenity_prelude as serenity,
};

use serenity::*;

use crate::{prelude::*, responses};

#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn nuke(context: BotContext<'_>, role: Role) -> BotResult<()> {
    // Defer reply since nuking multiple users will take a significant amount of time
    context.defer().await?;

    role.guild_id
        .members_iter(context.http())
        .try_filter(|member| future::ready(!member.user.bot && member.roles.contains(&role.id)))
        .map_err(BotError::from)
        .try_for_each_concurrent(10, |mut member| async move {
            let user_id = member.user.id;

            // Reset the user's registration state to started
            sqlx::query!(
                "update registrations set status = ? where user_id = ?",
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
        })
        .await?;

    // Reply after nuke
    context
        .send(|reply| reply.embed(responses::registration_nuke_embed(role.id)))
        .await?;

    Ok(())
}
