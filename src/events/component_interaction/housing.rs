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
    // Update the user's registration state to housing picked
    sqlx::query!(
        "update registrations set status = ? where user_id = ?",
        RegistrationStatus::HousingPicked.to_string(),
        component_interaction.user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Inform the user of completion
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::registration_finished(
                bot_data.channels.chat_channel_id,
            ))
        })
        .await?;

    Ok(())
}
