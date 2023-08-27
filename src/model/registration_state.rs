use std::str::FromStr;

use poise::serenity_prelude as serenity;

use serenity::UserId;

use sqlx::{mysql::MySqlRow, prelude::*, Error as SqlError};

use strum::{Display, EnumString};

use thiserror::Error;

use crate::prelude::*;

/// Registration data of a [`User`].
///
/// [`User`]: serenity::User
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Registration {
    /// The user's Discord user ID.
    pub user_id: UserId,
    /// Which stage of registration the user is currently in.
    pub status: RegistrationStatus,
}

impl Registration {
    /// Tries to construct a [`Registration`] from the provided database columns.
    ///
    /// # Errors
    ///
    /// See [`RegistrationStatus::from_columns`].
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

/// States of the registration process.
#[derive(Clone, Debug, Display, Default, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum RegistrationStatus {
    /// The user has not started registration yet.
    ///
    /// Users enter this stage when joining the guild or when restarting their registration.
    #[default]
    Unregistered,
    /// The user has started registration.
    Started,
    /// The user has entered (but not yet confirmed) their name.
    NameEntered(String),
    /// The user has confirmed their name, and at least one unregistered [`VerifiedUser`] with the same name is found.
    KindFound(String, VerifiedUserKind),
    /// The user has confirmed their [`VerifiedUserKind`] and is verified, but not registered.
    Verified,
    /// The user has picked their pronouns (or skipped the question).
    PronounsPicked,
    /// The user has finished registration.
    Registered,
    /// One of the previous registration steps has failed.
    ///
    /// Causes of failure include:
    /// - The user cannot be sent a direct message to start (or restart) registration
    /// - A matching unregistered [`VerifiedUser`] entry could not be found
    /// - The user indicates that their matched [`VerifiedUserKind`] is incorrect
    Failed,
}

impl RegistrationStatus {
    /// Tries to construct a [`RegistrationStatus`] from the provided database columns.
    ///
    /// # Errors
    ///
    /// Fails if `status` is not a valid [`RegistrationStatus`] variant name, or if the combination of `name` and `kind` is invalid for the given `status`.
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
            (Self::KindFound(..), Some(name), Some(kind)) => Ok(Self::KindFound(
                name,
                VerifiedUserKind::from_str(&kind)
                    .map_err(|_| DecodeRegistrationError::UnknownKind)?,
            )),
            (this, None, None) => Ok(this),
            (..) => Err(DecodeRegistrationError::InvalidDataCombination),
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
