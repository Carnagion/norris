use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn message_component_interacted(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Defer the response to allow time for database queries
    component_interaction.defer(&context.http).await?;

    match component_interaction.data.custom_id.as_str() {
        // User has started registration
        responses::INSTRUCTIONS_CONTINUE => {
            instructions_continue_clicked(context, component_interaction, bot_data).await
        },
        // User has confirmed their name
        responses::NAME_CONFIRM_YES => {
            name_confirmed(context, component_interaction, bot_data).await
        },
        // User wants to enter a different name
        responses::NAME_CONFIRM_NO => name_denied(context, component_interaction, bot_data).await,
        // User has confirmed their kind
        responses::KIND_CONFIRM_YES => {
            kind_confirmed(context, component_interaction, bot_data).await
        },
        // User has been incorrectly detected as the wrong kind
        responses::KIND_CONFIRM_NO => kind_denied(context, component_interaction, bot_data).await,
        // User has been registered and is continuing on to pronouns and housing
        responses::OPTIONAL_CONTINUE => ask_pronouns(context, component_interaction).await,
        _ => Ok(()),
    }
}

async fn instructions_continue_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration state to started
    sqlx::query!(
        "update registrations set status = ? where user_id = ?",
        RegistrationStatus::Started.to_string(),
        component_interaction.user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to enter their name
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::request_name_embed())
        })
        .await?;

    Ok(())
}

async fn name_confirmed(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Retrieve the user's name
    let user_name = sqlx::query!(
        "select name from registrations where user_id = ? limit 1",
        component_interaction.user.id.0,
    )
    .fetch_one(&bot_data.database_pool)
    .await?
    .name
    .unwrap(); // PANICS: This should have been set in the previous state and will always exist

    // Try to find a matching user
    let verified_user = sqlx::query!(
        "select * from users where name = ? and registered_user_id is null order by kind limit 1",
        user_name,
    )
    .try_map(|row| VerifiedUser::from_columns(row.name, row.kind, row.registered_user_id))
    .fetch_optional(&bot_data.database_pool)
    .await?;

    match verified_user {
        // No matching name was found
        None => no_name_error(context, component_interaction, bot_data).await,
        // Confirm the user's kind
        Some(verified_user) => {
            request_confirm_kind(context, component_interaction, bot_data, verified_user).await
        },
    }
}

async fn request_confirm_kind(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    verified_user: VerifiedUser,
) -> BotResult<()> {
    // Update the user's registration state to name confirmed
    sqlx::query!(
        "update registrations set status = ?, kind = ? where user_id = ?", // NOTE: Name should have been set when name entered
        RegistrationStatus::KindFound(verified_user.name, verified_user.kind).to_string(),
        verified_user.kind.to_string(),
        component_interaction.user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to confirm their kind
    component_interaction
        .create_followup_message(&context.http, |message| {
            message
                .embed(responses::confirm_kind_embed(verified_user.kind))
                .components(responses::confirm_kind_buttons())
        })
        .await?;

    Ok(())
}

async fn no_name_error(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration state to failed
    sqlx::query!(
        "update registrations set status = ?, name = null where user_id = ?",
        RegistrationStatus::Failed.to_string(),
        component_interaction.user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to seek assistance
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::no_name_error_embed(bot_data.support_channel_id))
        })
        .await?;

    Ok(())
}

async fn name_denied(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    let user = &component_interaction.user;

    // Update the user's registration state to started again
    sqlx::query!(
        "update registrations set status = ?, name = null where user_id = ?",
        RegistrationStatus::Started.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to enter their name
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::request_name_embed())
        })
        .await?;

    Ok(())
}

async fn kind_confirmed(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    let user = &component_interaction.user;

    let registration = sqlx::query!(
        "select name, kind from registrations where user_id = ? limit 1",
        user.id.0,
    )
    .fetch_one(&bot_data.database_pool)
    .await?;

    // Confirm the user as registered
    sqlx::query!(
        "update registrations set status = ?, name = null, kind = null where user_id = ?",
        RegistrationStatus::Registered.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Link the user's account to the verified user
    sqlx::query!(
        "update users set registered_user_id = ? where name = ? and kind = ? limit 1", // NOTE: Only one user should be registered
        user.id.0,
        registration.name,
        registration.kind,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Inform the user of completion and extra optional questions
    component_interaction
        .create_followup_message(&context.http, |message| {
            message
                .embed(responses::registered_continue_embed(
                    bot_data.arrival_channel_id,
                ))
                .components(responses::registered_continue_button())
        })
        .await?;

    Ok(())
}

async fn kind_denied(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    let user = &component_interaction.user;

    // Set the user's registration state to failed
    sqlx::query!(
        "update registrations set status = ?, name = null where user_id = ?",
        RegistrationStatus::Failed.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to seek assistance
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::kind_error_embed(bot_data.support_channel_id))
        })
        .await?;

    Ok(())
}

async fn ask_pronouns(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
) -> BotResult<()> {
    // Ask the user their pronouns
    component_interaction
        .create_followup_message(&context.http, |message| {
            message
                .embed(responses::pronouns_embed())
                .components(responses::pronouns_buttons())
        })
        .await?;

    Ok(())
}
