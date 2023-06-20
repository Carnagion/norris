use poise::{Command, Context, Framework};

mod data;
pub use data::*;

mod error;
pub use error::*;

pub mod model;

pub mod registration;

mod event_handler;
pub use event_handler::*;

pub type BotCommand = Command<BotData, BotError>;

pub type BotContext<'a> = Context<'a, BotData, BotError>;

pub type BotFramework = Framework<BotData, BotError>;
