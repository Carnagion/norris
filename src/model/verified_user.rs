use std::str::FromStr;

use poise::serenity_prelude as serenity;

use serenity::UserId;

use sqlx::{mysql::MySqlRow, prelude::*, Error as SqlError};

use strum::{Display, EnumString};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VerifiedUser {
    pub name: String,
    pub kind: VerifiedUserKind,
    pub registered_user_id: Option<UserId>,
}

impl VerifiedUser {
    fn from_columns(
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

#[derive(Clone, Copy, Debug, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum VerifiedUserKind {
    Undergrad,
    Postgrad,
    Mentor,
    SeniorMentor,
    HonoraryMentor,
    Faculty,
}

impl VerifiedUserKind {
    pub fn description(self) -> &'static str {
        match self {
            Self::Undergrad => "first-year undergraduate student",
            Self::Postgrad => "postgraduate student",
            Self::Mentor => "mentor",
            Self::SeniorMentor => "senior mentor",
            Self::HonoraryMentor => "honorary mentor",
            Self::Faculty => "member of faculty",
        }
    }
}
