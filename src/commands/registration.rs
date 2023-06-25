use crate::prelude::*;

pub mod nuke;
pub use nuke::nuke;

#[poise::command(slash_command, subcommands("nuke"), guild_only)]
pub async fn registration(_: BotContext<'_>) -> BotResult<()> {
    unreachable!() // PANICS: Will never be reached since Discord does not allow calling parent slash commands without subcommands
}
