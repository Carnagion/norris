use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

mod instructions;

mod name_confirm;

mod kind_confirm;

mod registered;

mod pronouns;

pub async fn message_component_interacted(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Defer the response to allow time for database queries
    component_interaction.defer(&context.http).await?;

    // Try to get the user's registration status
    let registration_status = sqlx::query!(
        "select status, name, kind from registrations where user_id = ?",
        component_interaction.user.id.0,
    )
    .try_map(|row| RegistrationStatus::from_columns(row.status, row.name, row.kind))
    .fetch_optional(&bot_data.database_pool)
    .await?;

    let component_id = component_interaction.data.custom_id.as_str();

    match (component_id, registration_status) {
        // User has started registration
        (responses::INSTRUCTIONS_CONTINUE, Some(RegistrationStatus::Unregistered)) => {
            instructions::continue_clicked(context, component_interaction, bot_data).await
        },
        // User has confirmed their name
        (responses::NAME_CONFIRM_YES, Some(RegistrationStatus::NameEntered(name))) => {
            name_confirm::yes_clicked(context, component_interaction, bot_data, name).await
        },
        // User wants to enter a different name
        (responses::NAME_CONFIRM_NO, Some(RegistrationStatus::NameEntered(_))) => {
            name_confirm::no_clicked(context, component_interaction, bot_data).await
        },
        // User has confirmed their kind
        (responses::KIND_CONFIRM_YES, Some(RegistrationStatus::KindFound(name, kind))) => {
            kind_confirm::yes_clicked(context, component_interaction, bot_data, name, kind).await
        },
        // User has been incorrectly detected as the wrong kind
        (responses::KIND_CONFIRM_NO, Some(RegistrationStatus::KindFound(_, _))) => {
            kind_confirm::no_clicked(context, component_interaction, bot_data).await
        },
        // User has been registered and is continuing on to pronouns and housing
        (responses::REGISTERED_CONTINUE, Some(RegistrationStatus::Registered)) => {
            registered::continue_clicked(context, component_interaction).await
        },
        // User has registered and is picking a pronoun
        (
            pronoun @ (responses::PRONOUNS_HE_HIM
            | responses::PRONOUNS_SHE_HER
            | responses::PRONOUNS_THEY_THEM
            | responses::PRONOUNS_XE_XEM
            | responses::PRONOUNS_ANY
            | responses::PRONOUNS_ASK),
            Some(RegistrationStatus::Registered),
        ) => pronouns::pronouns_clicked(context, component_interaction, bot_data, pronoun).await,
        // User has registered and does not want to pick a pronoun
        (responses::PRONONS_SKIP, Some(RegistrationStatus::Registered)) => {
            pronouns::pronouns_skip_clicked(context, component_interaction, bot_data).await
        },
        _ => Ok(()),
    }
}
