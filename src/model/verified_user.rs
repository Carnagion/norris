use std::str::FromStr;

use poise::serenity_prelude::UserId;

use sqlx::Error as SqlError;

use strum::{Display, EnumString};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VerifiedUser {
    pub name: String,
    pub kind: VerifiedUserKind,
    pub registered_user_id: Option<UserId>,
}

impl VerifiedUser {
    pub fn from_row(
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
