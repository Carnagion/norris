use poise::{serenity_prelude as serenity, Event};

use serenity::*;

use crate::prelude::*;

pub async fn event_handler(
    context: &Context,
    event: &Event<'_>,
    bot_data: &BotData,
) -> BotResult<()> {
    match event {
        Event::GuildMemberAddition { new_member } if !new_member.user.bot => {
            registration::setup_registration(context, new_member, bot_data).await
        },
        // Event::GuildMemberRemoval { user, .. } if !user.bot => {
        //     // registration::clear_registration(context, user, bot_data).await
        // },
        // Event::InteractionCreate {
        //     interaction: Interaction::MessageComponent(component_interaction),
        // } => {
        //     todo!()
        // },
        _ => Ok(()),
    }
}
