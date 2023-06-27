use crate::prelude::*;

mod undergrads;
pub use undergrads::undergrads;

mod postgrads;
pub use postgrads::postgrads;

#[poise::command(slash_command, subcommands("undergrads", "postgrads"), guild_only)]
pub async fn count(_: BotContext<'_>) -> BotResult<()> {
    unreachable!() // PANICS: Will never be reached since Discord does not allow calling parent slash commands without subcommands
}
