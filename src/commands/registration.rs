//! Registration-related slash commands.

use crate::prelude::*;

mod restart;
pub use restart::restart;

mod nuke;
pub use nuke::nuke;

mod add;
pub use add::add;

/// Modify registrations.
#[poise::command(slash_command, subcommands("restart", "nuke", "add"), guild_only)]
pub async fn registration(_: BotContext<'_>) -> BotResult<()> {
    unreachable!() // PANICS: Will never be reached since Discord does not allow calling parent slash commands without subcommands
}
