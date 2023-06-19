use std::str::FromStr;

use poise::serenity_prelude::UserId;

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

    pub(crate) fn from_parts(
        tag: &str,
        name: Option<String>,
    ) -> Result<Self, RegistrationFromPartsError> {
        let this = Self::from_str(tag).map_err(|_| RegistrationFromPartsError::UnknownStatus)?;
        match (this, name) {
            (Self::NameEntered(_), Some(name)) => Ok(Self::NameEntered(name)),
            (Self::NameConfirmed(_), Some(name)) => Ok(Self::NameConfirmed(name)),
            (Self::NameEntered(_) | Self::NameConfirmed(_), None) => {
                Err(RegistrationFromPartsError::MissingData)
            },
            (this, None) => Ok(this),
            (_, Some(_)) => Err(RegistrationFromPartsError::InvalidDataCombination),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum RegistrationFromPartsError {
    #[error("missing data associated with tag")]
    MissingData,
    #[error("invalid combination of tag and data")]
    InvalidDataCombination,
    #[error("unknown tag")]
    UnknownStatus,
}
