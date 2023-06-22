use std::str::FromStr;

use poise::serenity_prelude as serenity;

use serenity::UserId;

use sqlx::{mysql::MySqlRow, prelude::*, Error as SqlError};

use strum::{Display, EnumString};

use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Registration {
    pub user_id: UserId,
    pub status: RegistrationStatus,
}

#[derive(Clone, Debug, Default, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum RegistrationStatus {
    #[default]
    Unregistered,
    Started,
    NameEntered(String),
    NameConfirmed(String),
    Registered,
    PronounsPicked,
    HousingPicked,
    Failed,
}

impl RegistrationStatus {
    pub fn is_registered(&self) -> bool {
        matches!(
            self,
            Self::Registered | Self::PronounsPicked | Self::HousingPicked
        )
    }

    fn from_columns(status: String, name: Option<String>) -> Result<Self, SqlError> {
        Self::from_parts(&status, name).map_err(|error| SqlError::ColumnDecode {
            index: "status".to_owned(),
            source: Box::new(error),
        })
    }

    fn from_parts(status: &str, name: Option<String>) -> Result<Self, DecodeRegistrationError> {
        let this = Self::from_str(status).map_err(|_| DecodeRegistrationError::UnknownStatus)?;
        match (this, name) {
            (Self::NameEntered(_), Some(name)) => Ok(Self::NameEntered(name)),
            (Self::NameConfirmed(_), Some(name)) => Ok(Self::NameConfirmed(name)),
            (Self::NameEntered(_) | Self::NameConfirmed(_), None) => {
                Err(DecodeRegistrationError::MissingData)
            },
            (this, None) => Ok(this),
            (_, Some(_)) => Err(DecodeRegistrationError::InvalidDataCombination),
        }
    }
}

impl FromRow<'_, MySqlRow> for RegistrationStatus {
    fn from_row(row: &MySqlRow) -> Result<Self, SqlError> {
        Self::from_columns(row.try_get("status")?, row.try_get("name")?)
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
enum DecodeRegistrationError {
    #[error("missing data associated with status")]
    MissingData,
    #[error("invalid combination of status and data")]
    InvalidDataCombination,
    #[error("unknown status")]
    UnknownStatus,
}
