use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

mod instructions;

mod name_confirm;

mod kind_confirm;

mod verified;

mod pronouns;

mod housing;

pub async fn message_component_interacted(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Defer the response to allow time for database queries
    component_interaction.defer(&context.http).await?;

    // Try to get the user's registration status
    let registration_status = sqlx::query!(
        "select status, name, kind from registrations
        where user_id = ?",
        component_interaction.user.id.0,
    )
    .try_map(|row| RegistrationStatus::from_columns(row.status, row.name, row.kind))
    .fetch_optional(&bot_data.database_pool)
    .await?;

    let component_id = component_interaction.data.custom_id.as_str();

    match (component_id, registration_status) {
        // User has started registration
        (components::INSTRUCTIONS_CONTINUE, Some(RegistrationStatus::Unregistered)) => {
            instructions::continue_clicked(context, component_interaction, bot_data).await
        },
        // User has confirmed their name
        (components::NAME_CONFIRM_YES, Some(RegistrationStatus::NameEntered(name))) => {
            name_confirm::yes_clicked(context, component_interaction, bot_data, name).await
        },
        // User wants to enter a different name
        (components::NAME_CONFIRM_NO, Some(RegistrationStatus::NameEntered(_))) => {
            name_confirm::no_clicked(context, component_interaction, bot_data).await
        },
        // User has confirmed their kind
        (components::KIND_CONFIRM_YES, Some(RegistrationStatus::KindFound(name, kind))) => {
            kind_confirm::yes_clicked(context, component_interaction, bot_data, name, kind).await
        },
        // User has been incorrectly detected as the wrong kind
        (components::KIND_CONFIRM_NO, Some(RegistrationStatus::KindFound(_, kind))) => {
            kind_confirm::no_clicked(context, component_interaction, bot_data, kind).await
        },
        // User has been verified and is continuing on to pronouns and housing
        (components::VERIFIED_CONTINUE, Some(RegistrationStatus::Verified)) => {
            verified::continue_clicked(context, component_interaction).await
        },
        // User has been verified and is picking a pronoun
        (
            pronoun @ (components::PRONOUNS_HE_HIM
            | components::PRONOUNS_SHE_HER
            | components::PRONOUNS_THEY_THEM
            | components::PRONOUNS_XE_XEM
            | components::PRONOUNS_ANY
            | components::PRONOUNS_ASK),
            Some(RegistrationStatus::Verified),
        ) => pronouns::pronouns_clicked(context, component_interaction, bot_data, pronoun).await,
        // User has been verified and does not want to pick a pronoun
        (components::PRONOUNS_SKIP, Some(RegistrationStatus::Verified)) => {
            pronouns::skip_clicked(context, component_interaction, bot_data).await
        },
        // User has picked or skipped pronouns and is picking housing
        (
            housing @ (components::HOUSING_JC_CATERED
            | components::HOUSING_UP_CATERED
            | components::HOUSING_JC_SELF_CATERED
            | components::HOUSING_UP_SELF_CATERED
            | components::HOUSING_PRIVATE),
            Some(RegistrationStatus::PronounsPicked),
        ) => housing::housing_clicked(context, component_interaction, bot_data, housing).await,
        // User has picked or skipped pronouns and does not want to pick housing
        (components::HOUSING_SKIP, Some(RegistrationStatus::PronounsPicked)) => {
            housing::skip_clicked(context, component_interaction, bot_data).await
        },
        _ => Ok(()),
    }
}
