use poise::serenity_prelude::UserId;

use strum::{Display, EnumString};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VerifiedUser {
    pub name: String,
    pub kind: VerifiedUserKind,
    pub registered_user_id: Option<UserId>,
}

#[derive(Clone, Copy, Debug, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum VerifiedUserKind {
    #[strum(to_string = "first-year undergraduate student")]
    Undergrad,
    #[strum(to_string = "postgraduate student")]
    Postgrad,
    #[strum(to_string = "mentor")]
    Mentor,
    #[strum(to_string = "senior mentor")]
    SeniorMentor,
    #[strum(to_string = "honorary mentor")]
    HonoraryMentor,
    #[strum(to_string = "member of faculty")]
    Faculty,
}
