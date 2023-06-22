use poise::{serenity_prelude as serenity, Event};

use serenity::*;

use crate::prelude::*;

mod guild_member_addition;
pub use guild_member_addition::*;

mod guild_member_removal;
pub use guild_member_removal::*;

mod component_interaction;
pub use component_interaction::*;

pub async fn event_handler(
    context: &Context,
    event: &Event<'_>,
    bot_data: &BotData,
) -> BotResult<()> {
    match event {
        Event::GuildMemberAddition { new_member } if !new_member.user.bot => {
            guild_member_added(context, new_member, bot_data).await
        },
        Event::GuildMemberRemoval { user, .. } if !user.bot => {
            guild_member_removed(context, user, bot_data).await
        },
        Event::InteractionCreate {
            interaction: Interaction::MessageComponent(component_interaction),
        } => message_component_interacted(context, component_interaction, bot_data).await,
        Event::Message { new_message: _ } => {
            // TODO: Add logic for handling name entering
            todo!()
        },
        _ => Ok(()),
    }
}
