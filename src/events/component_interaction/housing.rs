use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn housing_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    housing: &str,
) -> BotResult<()> {
    // Get the role for the housing that was picked
    let housing_config = bot_data.roles.housing;
    let housing_role_id = match housing {
        responses::HOUSING_JC_CATERED => housing_config.jc_catered_role_id,
        responses::HOUSING_UP_CATERED => housing_config.up_catered_role_id,
        responses::HOUSING_JC_SELF_CATERED => housing_config.jc_self_catered_role_id,
        responses::HOUSING_UP_SELF_CATERED => housing_config.up_self_catered_role_id,
        responses::HOUSING_PRIVATE => housing_config.private_house_role_id,
        _ => unreachable!(), // PANICS: This function is only called with one of the above housing as input
    };

    // Give the user the desired housing role
    bot_data
        .guild_id
        .member(&context.http, component_interaction.user.id)
        .await?
        .add_role(&context.http, housing_role_id)
        .await?;

    // Finish registration completely
    skip_clicked(context, component_interaction, bot_data).await
}

pub async fn skip_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    let user_id = component_interaction.user.id;

    // Update the user's registration state to housing picked
    sqlx::query!(
        "update registrations set status = ? where user_id = ?",
        RegistrationStatus::Registered.to_string(),
        user_id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Find the matching verified user
    let verified_user = sqlx::query!(
        "select * from users
        where registered_user_id = ?", // NOTE: This should have been set when verified before pronouns
        user_id.0,
    )
    .try_map(|row| VerifiedUser::from_columns(row.name, row.kind, row.registered_user_id))
    .fetch_one(&bot_data.database_pool)
    .await?;

    let mut member = bot_data.guild_id.member(&context.http, user_id).await?;

    // Give the user the appropriate role
    member
        .add_role(
            &context.http,
            bot_data.roles.hierarchy.role(verified_user.kind),
        )
        .await?;

    // Change the user's nickname to their verified name
    member
        .edit(&context.http, |member| member.nickname(verified_user.name))
        .await?;

    // Inform the user of completion
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::registration_finished_embed(
                bot_data.channels.chat_channel_id,
            ))
        })
        .await?;

    // Log the completion of registration
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(responses::registered_log_embed(user_id))
        })
        .await?;

    Ok(())
}
