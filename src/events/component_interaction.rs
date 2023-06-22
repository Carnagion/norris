use poise::serenity_prelude as serenity;

use serenity::{colours::branding::BLURPLE, *};

use crate::prelude::*;

pub const INSTRUCTIONS_CONTINUE: &str = "instructions_continue";

pub async fn message_component_interacted(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    match component_interaction.data.custom_id.as_str() {
        INSTRUCTIONS_CONTINUE => {
            instructions_continue_clicked(context, &component_interaction.user, bot_data).await
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
    sqlx::query_file!(
        "queries/update-registration-state.sql",
        RegistrationStatus::Started.to_string(),
        user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to enter their name
    user.direct_message(&context.http, |message| message.embed(request_name_embed()))
        .await?;

    Ok(())
}

fn request_name_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed.title("Registration").colour(BLURPLE).description(
            "Please type out your **name** exactly as when you applied to the University.",
        )
    }
}
