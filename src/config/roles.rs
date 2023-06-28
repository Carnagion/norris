use poise::serenity_prelude as serenity;

use serde::{Deserialize, Serialize};

use serenity::*;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RolesConfig {
    pub hierarchy: HierarchyRolesConfig,
    pub pronouns: PronounRolesConfig,
    pub housing: HousingRolesConfig,
}

impl RolesConfig {
    pub fn roles_needing_registration(&self) -> [RoleId; 17] {
        [
            // Hierarchy and user kind roles
            self.hierarchy.undergrad_role_id,
            self.hierarchy.postgrad_role_id,
            self.hierarchy.mentor_role_id,
            self.hierarchy.senior_mentor_role_id,
            self.hierarchy.honorary_mentor_role_id,
            self.hierarchy.faculty_role_id,
            // Pronoun roles
            self.pronouns.he_him_role_id,
            self.pronouns.she_her_role_id,
            self.pronouns.they_them_role_id,
            self.pronouns.xe_xem_role_id,
            self.pronouns.any_pronouns_role_id,
            self.pronouns.ask_pronouns_role_id,
            // Housing roles
            self.housing.jc_catered_role_id,
            self.housing.up_catered_role_id,
            self.housing.jc_self_catered_role_id,
            self.housing.up_self_catered_role_id,
            self.housing.private_house_role_id,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct HierarchyRolesConfig {
    pub undergrad_role_id: RoleId,
    pub postgrad_role_id: RoleId,
    pub mentor_role_id: RoleId,
    pub senior_mentor_role_id: RoleId,
    pub honorary_mentor_role_id: RoleId,
    pub faculty_role_id: RoleId,
}

impl HierarchyRolesConfig {
    pub fn role(&self, kind: VerifiedUserKind) -> RoleId {
        match kind {
            VerifiedUserKind::Undergrad => self.undergrad_role_id,
            VerifiedUserKind::Postgrad => self.postgrad_role_id,
            VerifiedUserKind::Mentor => self.mentor_role_id,
            VerifiedUserKind::SeniorMentor => self.senior_mentor_role_id,
            VerifiedUserKind::HonoraryMentor => self.honorary_mentor_role_id,
            VerifiedUserKind::Faculty => self.faculty_role_id,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PronounRolesConfig {
    pub he_him_role_id: RoleId,
    pub she_her_role_id: RoleId,
    pub they_them_role_id: RoleId,
    pub xe_xem_role_id: RoleId,
    pub any_pronouns_role_id: RoleId,
    pub ask_pronouns_role_id: RoleId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct HousingRolesConfig {
    pub jc_catered_role_id: RoleId,
    pub jc_self_catered_role_id: RoleId,
    pub up_catered_role_id: RoleId,
    pub up_self_catered_role_id: RoleId,
    pub private_house_role_id: RoleId,
}
