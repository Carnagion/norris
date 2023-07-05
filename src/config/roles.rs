use poise::serenity_prelude as serenity;

use serde::{Deserialize, Serialize};

use serenity::*;

use crate::prelude::*;

/// [`Role`]-related configuration data and mappings.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RolesConfig {
    /// Configuration data related to the [`VerifiedUserKind`] hierarchy.
    pub hierarchy: HierarchyRolesConfig,
    /// Configuration data related to pronoun roles.
    pub pronouns: PronounRolesConfig,
    /// Configuration data related to housing roles.
    pub housing: HousingRolesConfig,
}

impl RolesConfig {
    /// The IDs of all [`Role`]s that can be granted by the bot during the registration process.
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

    /// The IDs of all roles whose members can be nuked as part of [`nuke`] testing.
    ///
    /// [`nuke`]: crate::commands::registration::nuke
    pub fn nukable_roles(&self) -> [RoleId; 3] {
        [
            self.hierarchy.undergrad_role_id,
            self.hierarchy.postgrad_role_id,
            self.hierarchy.mentor_role_id,
        ]
    }
}

/// [`Role`] configuration data related to the [`VerifiedUserKind`] hierarchy.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct HierarchyRolesConfig {
    /// The ID of the role given to undergraduates.
    pub undergrad_role_id: RoleId,
    /// The ID of the role given to postgraduates.
    pub postgrad_role_id: RoleId,
    /// The ID of the role given to mentors.
    pub mentor_role_id: RoleId,
    /// The ID of the role given to senior mentors.
    pub senior_mentor_role_id: RoleId,
    /// The ID of the role given to honorary mentors.
    pub honorary_mentor_role_id: RoleId,
    /// The ID of the role given to members of faculty.
    pub faculty_role_id: RoleId,
}

impl HierarchyRolesConfig {
    /// Gets the role ID for a particular [`VerifiedUserKind`].
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

/// Pronoun [`Role`]s configuration data.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PronounRolesConfig {
    /// The ID of the `he / him` pronoun role.
    pub he_him_role_id: RoleId,
    /// The ID of the `she / her` pronoun role.
    pub she_her_role_id: RoleId,
    /// The ID of the `they / them` pronoun role.
    pub they_them_role_id: RoleId,
    /// The ID of the `xe / xem` pronoun role.
    pub xe_xem_role_id: RoleId,
    /// The ID of the `any pronouns` pronoun role.
    pub any_pronouns_role_id: RoleId,
    /// The ID of the `ask me` pronoun role.
    pub ask_pronouns_role_id: RoleId,
}

/// Housing [`Role`]s configuration data.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct HousingRolesConfig {
    /// The ID of the role for catered accommodation on Jubilee.
    pub jc_catered_role_id: RoleId,
    /// The ID of the role for self-catered accommodation around Jubilee.
    pub jc_self_catered_role_id: RoleId,
    /// The ID of the role for catered accommodation on University Park.
    pub up_catered_role_id: RoleId,
    /// The ID of the role for self-catered accommodation around University Park.
    pub up_self_catered_role_id: RoleId,
    /// The ID of the role for private housing.
    pub private_house_role_id: RoleId,
}
