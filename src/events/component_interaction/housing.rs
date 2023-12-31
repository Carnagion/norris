use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

#[tracing::instrument(
    skip_all,
    fields(
        interaction_id = %component_interaction.id,
        user_id = %component_interaction.user.id,
        message_id = %component_interaction.message.id,
    ),
    err(Debug),
)]
pub async fn housing_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    housing: &str,
) -> BotResult<()> {
    // Get the role for the housing that was picked
    let housing_config = bot_data.roles.housing;
    let housing_role_id = match housing {
        components::HOUSING_JC_CATERED => housing_config.jc_catered_role_id,
        components::HOUSING_UP_CATERED => housing_config.up_catered_role_id,
        components::HOUSING_JC_SELF_CATERED => housing_config.jc_self_catered_role_id,
        components::HOUSING_UP_SELF_CATERED => housing_config.up_self_catered_role_id,
        components::HOUSING_PRIVATE => housing_config.private_house_role_id,
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

#[tracing::instrument(
    skip_all,
    fields(
        interaction_id = %component_interaction.id,
        user_id = %component_interaction.user.id,
        message_id = %component_interaction.message.id,
    ),
    err(Debug),
)]
pub async fn skip_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    let user_id = component_interaction.user.id;

    // Update the user's registration state to housing picked
    sqlx::query!(
        "update registrations
        set status = ?
        where user_id = ?",
        RegistrationStatus::Registered.to_string(),
        user_id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Find the matching verified user
    let verified_user = sqlx::query!(
        "select * from users
        where registered_user_id = ?
        limit 1", // NOTE: This should have been set when verified before pronouns
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

    // Use a different channel for postgrads
    let main_channel_id = match verified_user.kind {
        VerifiedUserKind::Postgrad => bot_data.channels.postgrad.main_channel_id,
        _ => bot_data.channels.undergrad.main_channel_id,
    };

    // Inform the user of completion
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(embeds::registration::finished(main_channel_id))
        })
        .await?;

    // Welcome the user
    main_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::registration::welcome(user_id))
        })
        .await?;

    // Log the completion of registration
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message
                .add_embed(embeds::logs::housing_picked(user_id))
                .add_embed(embeds::logs::registration_finished(user_id))
        })
        .await?;

    Ok(())
}
