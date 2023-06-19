use poise::serenity_prelude::UserId;

use serde::{Deserialize, Serialize};

use sqlx::{types::Json, FromRow};

#[derive(Clone, Debug, Deserialize, Eq, FromRow, Hash, PartialEq, Serialize)]
pub struct OngoingRegistration {
    pub user_id: UserId,
    pub status: Json<RegistrationStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum RegistrationStatus {
    #[default]
    Unregistered,
    Started,
    NameEntered(String),
    NameConfirmed(String),
    Registered,
    PronounsPicked,
    HousingPicked,
    Failed(RegistrationFailure),
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum RegistrationFailure {
    NameNotFound,
    WrongNameEntered,
    WrongKindDetected,
}

impl RegistrationStatus {
    pub fn is_registered(&self) -> bool {
        matches!(
            self,
            Self::Registered | Self::PronounsPicked | Self::HousingPicked
        )
    }
}
