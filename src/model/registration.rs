use std::str::FromStr;

use poise::serenity_prelude::UserId;

use sqlx::Error as SqlError;

use strum::EnumString;

use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OngoingRegistration {
    pub user_id: UserId,
    pub status: RegistrationStatus,
}

#[derive(Clone, Debug, Default, EnumString, Eq, Hash, PartialEq)]
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

    pub fn from_row(tag: String, name: Option<String>) -> Result<Self, SqlError> {
        Self::from_parts(&tag, name).map_err(|error| SqlError::ColumnDecode {
            index: "status".to_owned(),
            source: Box::new(error),
        })
    }

    fn from_parts(tag: &str, name: Option<String>) -> Result<Self, DecodeRegistrationError> {
        let this = Self::from_str(tag).map_err(|_| DecodeRegistrationError::UnknownStatus)?;
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

#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum DecodeRegistrationError {
    #[error("missing data associated with status")]
    MissingData,
    #[error("invalid combination of status and data")]
    InvalidDataCombination,
    #[error("unknown status")]
    UnknownStatus,
}
