//! Re-exports of commonly-used items.

use poise::{Command, Context, Framework, FrameworkError};

pub use crate::{config::*, data::*, error::*, model::*, Norris};

pub(crate) use crate::responses::{components, embeds};

/// Result type returned by many of the bot's functions.
pub type BotResult<T> = Result<T, BotError>;

/// Framework type of the bot, parametrized by [`BotData`] and [`BotError`].
pub type BotFramework = Framework<BotData, BotError>;

/// Context type of the bot, parametrized by [`BotData`] and [`BotError`].
pub type BotContext<'a> = Context<'a, BotData, BotError>;

/// Command type used by the bot, parametrized by [`BotData`] and [`BotError`].
pub type BotCommand = Command<BotData, BotError>;

/// Framework error type of the bot, parametrized by [`BotData`] and [`BotError`].
pub type BotFrameworkError<'a> = FrameworkError<'a, BotData, BotError>;
