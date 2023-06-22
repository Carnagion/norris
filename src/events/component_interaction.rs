use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn message_component_interacted(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    match component_interaction.data.custom_id.as_str() {
        // User has started registration
        responses::INSTRUCTIONS_CONTINUE => {
            instructions_continue_clicked(context, &component_interaction.user, bot_data).await
        },
        responses::NAME_CONFIRM_YES => {
            name_confirmed(context, &component_interaction.user, bot_data).await
        },
        // User wants to enter a different name
        responses::NAME_CONFIRM_NO => {
            name_denied(context, &component_interaction.user, bot_data).await
        },
        // User has confirmed their kind
        responses::KIND_CONFIRM_YES => {
            kind_confirmed(context, &component_interaction.user, bot_data).await
        },
        // User has been incorrectly detected as the wrong kind
        responses::KIND_CONFIRM_NO => {
            kind_denied(context, &component_interaction.user, bot_data).await
        },
        _ => Ok(()),
    }
}

async fn instructions_continue_clicked(
    context: &Context,
    user: &User,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration state to started
    sqlx::query!(
        "update registrations set status = ? where user_id = ?",
        RegistrationStatus::Started.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to enter their name
    user.direct_message(&context.http, |message| {
        message.embed(responses::request_name_embed())
    })
    .await?;

    Ok(())
}

async fn name_confirmed(context: &Context, user: &User, bot_data: &BotData) -> BotResult<()> {
    // Update the user's registration state to name confirmed
    sqlx::query!(
        "update registrations set status = ? where user_id = ?", // NOTE: Name should have been set when name entered
        RegistrationStatus::NameConfirmed("".into()).to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Retrieve the user's name
    let user_name = sqlx::query!(
        "select name from registrations where user_id = ?",
        user.id.0,
    )
    .fetch_one(&bot_data.database_pool)
    .await?
    .name
    .unwrap(); // PANICS: This should have been set in the previous state and will always exist

    // Try to find a matching name
    let verified_user = sqlx::query!(
        "select * from users where name = ? and registered_user_id = null order by kind limit 1",
        user_name,
    )
    .try_map(|row| VerifiedUser::from_columns(row.name, row.kind, row.registered_user_id))
    .fetch_optional(&bot_data.database_pool)
    .await?;

    match verified_user {
        // Ask the user to confirm their kind
        Some(verified_user) => {
            user.direct_message(&context.http, |message| {
                message
                    .embed(responses::confirm_kind_embed(verified_user.kind))
                    .components(responses::confirm_kind_buttons())
            })
            .await?;
        },
        // No matching name was found
        None => {},
    }

    Ok(())
}

async fn name_denied(context: &Context, user: &User, bot_data: &BotData) -> BotResult<()> {
    // Update the user's registration state to started again
    sqlx::query!(
        "update registrations set status = ?, name = null where user_id = ?",
        RegistrationStatus::Started.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to enter their name
    user.direct_message(&context.http, |message| {
        message.embed(responses::request_name_embed())
    })
    .await?;

    Ok(())
}

async fn kind_confirmed(context: &Context, user: &User, bot_data: &BotData) -> BotResult<()> {
    Ok(())
}

async fn kind_denied(context: &Context, user: &User, bot_data: &BotData) -> BotResult<()> {
    sqlx::query!(
        "update registrations set status = ?, name = null where user_id = ?",
        RegistrationStatus::Failed.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    Ok(())
}
