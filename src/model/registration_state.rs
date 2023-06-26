use std::str::FromStr;

use poise::serenity_prelude as serenity;

use serenity::UserId;

use sqlx::{mysql::MySqlRow, prelude::*, Error as SqlError};

use strum::{Display, EnumString};

use thiserror::Error;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Registration {
    pub user_id: UserId,
    pub status: RegistrationStatus,
}

impl Registration {
    pub fn from_columns(
        user_id: u64,
        status: String,
        name: Option<String>,
        kind: Option<String>,
    ) -> Result<Self, SqlError> {
        let status = RegistrationStatus::from_columns(status, name, kind)?;
        Ok(Self {
            user_id: UserId(user_id),
            status,
        })
    }
}

#[derive(Clone, Debug, Display, Default, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistrationStatus {
    #[default]
    Unregistered,
    Started,
    NameEntered(String),
    KindFound(String, VerifiedUserKind),
    Verified,
    PronounsPicked,
    Registered,
    Failed,
}

impl RegistrationStatus {
    pub fn from_columns(
        status: String,
        name: Option<String>,
        kind: Option<String>,
    ) -> Result<Self, SqlError> {
        Self::from_parts(&status, name, kind).map_err(|error| SqlError::ColumnDecode {
            index: "status".to_owned(),
            source: Box::new(error),
        })
    }

    fn from_parts(
        status: &str,
        name: Option<String>,
        kind: Option<String>,
    ) -> Result<Self, DecodeRegistrationError> {
        let this = Self::from_str(status).map_err(|_| DecodeRegistrationError::UnknownStatus)?;
        match (this, name, kind) {
            (Self::NameEntered(_), Some(name), None) => Ok(Self::NameEntered(name)),
            (Self::KindFound(_, _), Some(name), Some(kind)) => Ok(Self::KindFound(
                name,
                VerifiedUserKind::from_str(&kind)
                    .map_err(|_| DecodeRegistrationError::UnknownKind)?,
            )),
            (this, None, None) => Ok(this),
            (_, _, _) => Err(DecodeRegistrationError::InvalidDataCombination),
        }
    }
}

impl FromRow<'_, MySqlRow> for Registration {
    fn from_row(row: &MySqlRow) -> Result<Self, SqlError> {
        Self::from_columns(
            row.try_get("user_id")?,
            row.try_get("status")?,
            row.try_get("name")?,
            row.try_get("kind")?,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
enum DecodeRegistrationError {
    #[error("invalid combination of status and data")]
    InvalidDataCombination,
    #[error("unknown status")]
    UnknownStatus,
    #[error("unknown user kind")]
    UnknownKind,
}
