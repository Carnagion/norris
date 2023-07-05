use poise::{
    futures_util::{future, TryStreamExt},
    serenity_prelude as serenity,
};

use serenity::*;

use crate::prelude::*;

/// Restart the registrations of multiple users.
#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn nuke(
    context: BotContext<'_>,
    #[description = "Only restart the registrations of users with this role."] role: Option<Role>,
) -> BotResult<()> {
    // Defer reply since nuking multiple users will take a significant amount of time
    context.defer().await?;

    let roles = role
        .map(|role| vec![role.id])
        .unwrap_or(Vec::from(context.data().roles.nukable_roles()));

    // Restart registrations of all non-bot members with the roles
    context
        .guild_id()
        .unwrap() // PANICS: This is a guild-only command and will always be executed in a guild
        .members_iter(context.http())
        .try_filter(|member| future::ready(!member.user.bot && roles.iter().any(|role| member.roles.contains(role))))
        .map_err(BotError::from)
        .try_for_each_concurrent(10, |mut member| async move {
            super::restart::restart_registration(context, &mut member).await
        })
        .await?;

    // Reply after nuke
    context
        .send(|reply| reply.embed(embeds::registration::nuke()))
        .await?;

    Ok(())
}
