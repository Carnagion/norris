use poise::{serenity_prelude::Context, Event};

use crate::{registration, BotData, BotError};

pub async fn event_handler(
    context: &Context,
    event: &Event<'_>,
    bot_data: &BotData,
) -> Result<(), BotError> {
    match event {
        Event::GuildMemberAddition { new_member } => {
            registration::setup_registration(context, new_member, bot_data).await?
        },
        Event::GuildMemberRemoval { user, .. } => {
            registration::clear_registration(context, user, bot_data).await?
        },
        _ => {},
    }

    Ok(())
}
