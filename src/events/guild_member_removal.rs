use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

pub async fn guild_member_removed(
    _context: &Context, // TODO: Will be used when registration logging is implemented
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

    Ok(())
}
