use poise::serenity_prelude as serenity;

use serenity::{colours::branding::BLURPLE, *};

use crate::{events::INSTRUCTIONS_CONTINUE, prelude::*};

pub async fn messaged(context: &Context, message: &Message, bot_data: &BotData) -> BotResult<()> {
    // Try and find a user with a matching name
    let verified_user = sqlx::query_file!("queries/select-user-name.sql", message.content)
        .try_map(|row| VerifiedUser::from_columns(row.name, row.kind, row.registered_user_id))
        .fetch_optional(&bot_data.database_pool)
        .await?;

    match verified_user {
        Some(_) => {},
        None => {
            notify_no_matching_name(context, message, bot_data).await?;
        },
    }

    Ok(())
}

async fn notify_no_matching_name(
    context: &Context,
    name_message: &Message,
    bot_data: &BotData,
) -> BotResult<()> {
    name_message
        .channel_id
        .send_message(&context.http, |message| message)
        .await?;
    Ok(())
}
