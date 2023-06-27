use poise::serenity_prelude as serenity;

use serenity::*;

use crate::{prelude::*, responses};

pub async fn continue_clicked(
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
