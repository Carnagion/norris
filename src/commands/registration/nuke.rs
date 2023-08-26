use poise::{
    futures_util::{future, TryStreamExt},
    serenity_prelude as serenity,
};

use serenity::*;

use crate::prelude::*;

/// Restart the registrations of multiple users.
#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
#[tracing::instrument(skip_all, fields(role_id = ?role.as_ref().map(|role| role.id)), err(Debug))]
pub async fn nuke(
    context: BotContext<'_>,
    #[description = "Only restart the registrations of users with this role."] role: Option<Role>,
) -> BotResult<()> {
    // Defer reply since nuking multiple users will take a significant amount of time
    context.defer().await?;

    let nukable_roles = role
        .map(|role| vec![role.id])
        .unwrap_or(Vec::from(context.data().roles.nukable_roles()));

    // Restart registrations of all non-bot members with the roles
    context
        .guild_id()
        .unwrap() // PANICS: This is a guild-only command and will always be executed in a guild
        .members_iter(context.http())
        .try_filter(|member| future::ready(can_nuke_member(member, &nukable_roles)))
        .map_err(BotError::from)
        // TODO: Figure out a better limit or spawn separate tasks, see https://github.com/Carnagion/norris/issues/2 and https://github.com/Carnagion/norris/issues/4
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

fn can_nuke_member(member: &Member, nukable_roles: &[RoleId]) -> bool {
    // Nuke members who have no roles or have at least one of the nukable roles
    !member.user.bot
        && (member.roles.is_empty() || nukable_roles.iter().any(|role| member.roles.contains(role)))
}
