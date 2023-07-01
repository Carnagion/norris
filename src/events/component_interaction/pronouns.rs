use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

pub async fn pronouns_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    pronouns: &str,
) -> BotResult<()> {
    // Get the role for the pronoun that was picked
    let pronouns_config = bot_data.roles.pronouns;
    let pronouns_role_id = match pronouns {
        components::PRONOUNS_HE_HIM => pronouns_config.he_him_role_id,
        components::PRONOUNS_SHE_HER => pronouns_config.she_her_role_id,
        components::PRONOUNS_THEY_THEM => pronouns_config.they_them_role_id,
        components::PRONOUNS_XE_XEM => pronouns_config.xe_xem_role_id,
        components::PRONOUNS_ANY => pronouns_config.any_pronouns_role_id,
        components::PRONOUNS_ASK => pronouns_config.ask_pronouns_role_id,
        _ => unreachable!(), // PANICS: This function is only called with one of the above pronouns as input
    };

    // Give the user the desired pronouns role
    bot_data
        .guild_id
        .member(&context.http, component_interaction.user.id)
        .await?
        .add_role(&context.http, pronouns_role_id)
        .await?;

    // Move on to housing
    skip_clicked(context, component_interaction, bot_data).await
}

pub async fn skip_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    let user_id = component_interaction.user.id;

    // Update the user's registration state to pronouns picked
    sqlx::query!(
        "update registrations
        set status = ?
        where user_id = ?",
        RegistrationStatus::PronounsPicked.to_string(),
        user_id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to pick housing
    component_interaction
        .create_followup_message(&context.http, |message| {
            message
                .embed(embeds::registration::housing())
                .components(components::housing_buttons())
        })
        .await?;

    // Log selection of pronouns
    bot_data
        .channels
        .log_channel_id
        .send_message(&context.http, |message| {
            message.embed(embeds::logs::pronouns_picked(user_id))
        })
        .await?;

    Ok(())
}
