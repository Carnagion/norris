use poise::{Command, Context, Framework, FrameworkError};

pub use crate::{config::*, data::*, error::*, model::*, Norris};

pub(crate) use crate::responses::{components, embeds};

pub type BotResult<T> = Result<T, BotError>;

pub type BotFramework = Framework<BotData, BotError>;

pub type BotContext<'a> = Context<'a, BotData, BotError>;

pub type BotCommand = Command<BotData, BotError>;

pub type BotFrameworkError<'a> = FrameworkError<'a, BotData, BotError>;
