use poise::serenity_prelude::{Context, User};

use crate::{BotData, BotError};

pub async fn clear_registration(
    context: &Context,
    user: &User,
    bot_data: &BotData,
) -> Result<(), BotError> {
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
