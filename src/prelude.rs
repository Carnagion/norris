use poise::{Command, Context, Framework};

pub use crate::{config::*, data::*, error::*, model::*, Norris};

pub type BotResult<T> = Result<T, BotError>;

pub type BotFramework = Framework<BotData, BotError>;

pub type BotContext<'a> = Context<'a, BotData, BotError>;

pub type BotCommand = Command<BotData, BotError>;
