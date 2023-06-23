use poise::serenity_prelude as serenity;

use serenity::Error as SerenityError;

use sqlx::Error as SqlError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("discord error: {}", .0)]
    Discord(#[from] SerenityError),
    #[error("{}", .0)]
    Sql(#[from] SqlError),
}
