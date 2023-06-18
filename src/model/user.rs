use std::fmt::{Display, Formatter, Result as FmtResult};

use poise::serenity_prelude::UserId;

use serde::{Deserialize, Serialize};

use sqlx::FromRow;

#[derive(Clone, Debug, Deserialize, Eq, FromRow, Hash, PartialEq, Serialize)]
pub struct VerifiedUser {
    pub name: String,
    pub kind: VerifiedUserKind,
    pub registered_user_id: Option<UserId>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum VerifiedUserKind {
    Undergrad,
    Postgrad,
    Mentor,
    SeniorMentor,
    HonoraryMentor,
    Faculty,
}

impl Display for VerifiedUserKind {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        let string = match self {
            Self::Undergrad => "first-year undergraduate student",
            Self::Postgrad => "postgraduate student",
            Self::Mentor => "mentor",
            Self::SeniorMentor => "senior mentor",
            Self::HonoraryMentor => "honorary mentor",
            Self::Faculty => "member of faculty",
        };
        write!(formatter, "{}", string)
    }
}
