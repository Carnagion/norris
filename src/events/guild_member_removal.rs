use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

/// Called when a [`Member`] leaves the [`Guild`].
#[tracing::instrument(skip_all, fields(user_id = %user.id), err(Debug))]
pub async fn guild_member_removed(
    context: &Context,
    user: &User,
    bot_data: &BotData,
) -> BotResult<()> {
    // Remove the registered user ID of the member from the table of verified users
    sqlx::query!(
        "update users
        set registered_user_id = null
        where registered_user_id = ?
        limit 1", // NOTE: Only a single user should be deleted
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Remove any registration information about the member
    sqlx::query!(
        "delete from registrations
        where user_id = ?",
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Log the user's exit
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::logs::user_left(user.id))
        })
        .await?;

    Ok(())
}
