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

    // Restart registrations of all non-bot members with the role
    role.guild_id
        .members_iter(context.http())
        .try_filter(|member| future::ready(!member.user.bot && member.roles.contains(&role.id)))
        .map_err(BotError::from)
        .try_for_each_concurrent(10, |mut member| async move {
            super::restart::restart_registration(context, &mut member).await
        })
        .await?;

    // Reply after nuke
    context
        .send(|reply| reply.embed(responses::registration_nuke_embed(role.id)))
        .await?;

    Ok(())
}
