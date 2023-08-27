use std::str::FromStr;

use poise::serenity_prelude as serenity;

use serenity::UserId;

use sqlx::{mysql::MySqlRow, prelude::*, Error as SqlError};

use strum::{Display, EnumString};

/// Data about a user who is expected to join, along with their Discord user ID if they have completed registration.
///
/// During registration, users are validated against a database of [`VerifiedUser`]s using their confirmed name and [`VerifiedUserKind`].
/// Those without a matching unregistered entry fail registration.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VerifiedUser {
    /// The user's full name, exactly as they applied to the University with.
    pub name: String,
    /// What kind of user this is.
    pub kind: VerifiedUserKind,
    /// The user's Discord user ID if they have registered.
    ///
    /// This is [`None`] if the user has not completed registration.
    pub registered_user_id: Option<UserId>,
}

impl VerifiedUser {
    /// Tries to construct a [`VerifiedUser`] from the provided database columns.
    ///
    /// # Errors
    ///
    /// Fails if `kind` is not a valid string representation of a [`VerifiedUserKind`].
    pub fn from_columns(
        name: String,
        kind: String,
        registered_user_id: Option<u64>,
    ) -> Result<Self, SqlError> {
        let user = Self {
            name,
            kind: VerifiedUserKind::from_str(&kind).map_err(|err| SqlError::ColumnDecode {
                index: "kind".to_owned(),
                source: Box::new(err),
            })?,
            registered_user_id: registered_user_id.map(UserId),
        };
        Ok(user)
    }
}

impl FromRow<'_, MySqlRow> for VerifiedUser {
    fn from_row(row: &MySqlRow) -> Result<Self, SqlError> {
        Self::from_columns(
            row.try_get("name")?,
            row.try_get("kind")?,
            row.try_get("registered_user_id")?,
        )
    }
}

/// Represents the various kinds of [`VerifiedUser`]s.
#[derive(Clone, Copy, Debug, Default, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum VerifiedUserKind {
    /// An undergraduate student.
    #[default]
    Undergrad,
    /// A postgraduate student.
    Postgrad,
    /// A mentor.
    Mentor,
    /// A senior mentor.
    SeniorMentor,
    /// An honorary mentor.
    HonoraryMentor,
    /// A member of faculty.
    Faculty,
}

impl VerifiedUserKind {
    /// Describes the [`VerifiedUserKind`] in a user-friendly, human-readable way.
    pub fn description(self) -> &'static str {
        match self {
            Self::Undergrad => "first-year undergraduate student",
            Self::Postgrad => "first-year postgraduate student",
            Self::Mentor => "mentor",
            Self::SeniorMentor => "senior mentor",
            Self::HonoraryMentor => "honorary mentor",
            Self::Faculty => "member of faculty",
        }
    }
}
