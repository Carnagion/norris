use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn guild_member_removed(
    context: &Context,
    user: &User,
    bot_data: &BotData,
) -> BotResult<()> {
    // Remove the registered user ID of the member from the table of verified users
    sqlx::query!(
        "update users set registered_user_id = null where registered_user_id = ?",
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Remove any registration information about the member
    sqlx::query!("delete from registrations where user_id = ?", user.id.0)
        .execute(&bot_data.database_pool)
        .await?;

    // Log the user's exit
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(responses::user_left_log_embed(user.id))
        })
        .await?;

    Ok(())
}
