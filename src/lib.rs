use poise::{Command, Context, Framework};

mod data;
pub use data::*;

mod error;
pub use error::*;

pub mod model;

pub type BotCommand = Command<BotData, BotError>;

pub type BotContext<'a> = Context<'a, BotData, BotError>;

pub type BotFramework = Framework<BotData, BotError>;
