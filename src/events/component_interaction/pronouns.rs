use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn pronouns_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    pronouns: &str,
) -> BotResult<()> {
    // Get the role for the pronoun that was picked
    let pronouns_config = bot_data.roles.pronouns;
    let pronouns_role_id = match pronouns {
        responses::PRONOUNS_HE_HIM => pronouns_config.he_him_role_id,
        responses::PRONOUNS_SHE_HER => pronouns_config.she_her_role_id,
        responses::PRONOUNS_THEY_THEM => pronouns_config.they_them_role_id,
        responses::PRONOUNS_XE_XEM => pronouns_config.xe_xem_role_id,
        responses::PRONOUNS_ANY => pronouns_config.any_pronouns_role_id,
        responses::PRONOUNS_ASK => pronouns_config.ask_pronouns_role_id,
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
    pronouns_skip_clicked(context, component_interaction, bot_data).await
}

pub async fn pronouns_skip_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration state to pronouns picked
    sqlx::query!(
        "update registrations set status = ? where user_id = ?",
        RegistrationStatus::PronounsPicked.to_string(),
        component_interaction.user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to pick housing
    component_interaction
        .create_followup_message(&context.http, |message| {
            message.embed(responses::housing_embed())
        })
        .await?;

    Ok(())
}
