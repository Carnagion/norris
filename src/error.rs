use poise::serenity_prelude::SerenityError;

use sqlx::Error as SqlError;

use thiserror::Error;

use crate::model::DecodeRegistrationError;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("{}", .0)]
    Discord(#[from] SerenityError),
    #[error("{}", .0)]
    Sql(#[from] SqlError),
    #[error("could not decode registration status ({})", .0)]
    RegistrationParse(#[from] DecodeRegistrationError),
}
