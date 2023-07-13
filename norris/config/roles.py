"""
Role-related configuration types.
"""

from dataclasses import dataclass
from enum import Enum

from serde import serde

from ..model import VerifiedUserKind


@serde(rename_all="kebabcase")
@dataclass
class HierarchyRolesConfig:
    """
    Role configuration data related to the
    `norris.model.verified_user.VerifiedUserKind` hierarchy.
    """

    undergrad_role_id: int
    """
    The ID of the role given to undergraduates.
    """

    postgrad_role_id: int
    """
    The ID of the role given to postgraduates.
    """

    mentor_role_id: int
    """
    The ID of the role given to mentors.
    """

    senior_mentor_role_id: int
    """
    The ID of the role given to senior mentors.
    """

    honorary_mentor_role_id: int
    """
    The ID of the role given to honorary mentors.
    """

    faculty_role_id: int
    """
    The ID of the role given to members of faculty.
    """

    def role_id(self, kind: VerifiedUserKind) -> int:
        """
        Gets the role ID for a particular `norris.model.verified_user.VerifiedUserKind`.
        """
        match kind:
            case VerifiedUserKind.UNDERGRAD:
                return self.undergrad_role_id
            case VerifiedUserKind.POSTGRAD:
                return self.postgrad_role_id
            case VerifiedUserKind.MENTOR:
                return self.mentor_role_id
            case VerifiedUserKind.SENIOR_MENTOR:
                return self.senior_mentor_role_id
            case VerifiedUserKind.HONORARY_MENTOR:
                return self.honorary_mentor_role_id
            case VerifiedUserKind.FACULTY:
                return self.faculty_role_id


class Pronouns(Enum):
    """
    Pronouns recognised by the bot.
    """

    HE_HIM = "he_him"
    """
    `He / Him` pronouns.
    """

    SHE_HER = "she_her"
    """
    `She / Her` pronouns.
    """

    THEY_THEM = "they_them"
    """
    `They / Them` pronouns.
    """

    XE_XEM = "xe_xem"
    """
    `Xe / Xem` pronouns.
    """

    ANY = "any"
    """
    Any pronouns are acceptable.
    """

    ASK = "ask"
    """
    Ask the user their pronouns.
    """


@serde(rename_all="kebabcase")
@dataclass
class PronounRolesConfig:
    """
    Pronoun roles configuration data.
    """

    he_him_role_id: int
    """
    The ID of the `he / him` pronoun role.
    """

    she_her_role_id: int
    """
    The ID of the `she / her` pronoun role.
    """

    they_them_role_id: int
    """
    The ID of the `they / them` pronoun role.
    """

    xe_xem_role_id: int
    """
    The ID of the `xe / xem` pronoun role.
    """

    any_pronouns_role_id: int
    """
    The ID of the `any pronouns` pronoun role.
    """

    ask_pronouns_role_id: int
    """
    The ID of the `ask me` pronoun role.
    """

    def role_id(self, pronouns: Pronouns) -> int:
        """
        Gets the role ID for a particular `Pronouns`.
        """
        match pronouns:
            case Pronouns.HE_HIM:
                return self.he_him_role_id
            case Pronouns.SHE_HER:
                return self.she_her_role_id
            case Pronouns.THEY_THEM:
                return self.they_them_role_id
            case Pronouns.XE_XEM:
                return self.xe_xem_role_id
            case Pronouns.ANY:
                return self.any_pronouns_role_id
            case Pronouns.ASK:
                return self.ask_pronouns_role_id


class Housing(Enum):
    """
    Housing options recognised by the bot.
    """

    JC_CATERED = "JC_CATERED"
    """
    Catered accommodation on Jubilee.
    """

    JC_SELF_CATERED = "JC_SELF_CATERED"
    """
    Self-catered accommodation around Jubilee.
    """

    UP_CATERED = "UP_CATERED"
    """
    Catered accommodation on University Park.
    """

    UP_SELF_CATERED = "UP_SELF_CATERED"
    """
    Self-catered accommodation around University Park.
    """

    PRIVATE_HOUSE = "PRIVATE_HOUSE"
    """
    Private housing.
    """


@serde(rename_all="kebabcase")
@dataclass
class HousingRolesConfig:
    """
    Housing roles configuration data.
    """

    jc_catered_role_id: int
    """
    The ID of the role for catered accommodation on Jubilee.
    """

    jc_self_catered_role_id: int
    """
    The ID of the role for self-catered accommodation around Jubilee.
    """

    up_catered_role_id: int
    """
    The ID of the role for catered accommodation on University Park.
    """

    up_self_catered_role_id: int
    """
    The ID of the role for self-catered accommodation around University Park.
    """

    private_house_role_id: int
    """
    The ID of the role for private housing.
    """

    def role_id(self, housing: Housing) -> int:
        """
        Gets the role ID for a particular `Housing`.
        """
        match housing:
            case Housing.JC_CATERED:
                return self.jc_catered_role_id
            case Housing.JC_SELF_CATERED:
                return self.jc_self_catered_role_id
            case Housing.UP_CATERED:
                return self.up_catered_role_id
            case Housing.UP_SELF_CATERED:
                return self.up_self_catered_role_id
            case Housing.PRIVATE_HOUSE:
                return self.private_house_role_id


@serde(rename_all="kebabcase")
@dataclass
class RolesConfig:
    """
    Role-related configuration data and mappings.
    """

    hierarchy: HierarchyRolesConfig
    """
    Configuration data related to the `norris.model.verified_user.VerifiedUserKind`
    hierarchy.
    """

    pronouns: PronounRolesConfig
    """
    Configuration data related to pronoun roles.
    """

    housing: HousingRolesConfig
    """
    Configuration data related to housing roles.
    """

    def nukeable_role_ids(self) -> list[int]:
        """
        The IDs of all roles whose members can be nuked as part of nuke testing.
        """
        return [
            self.hierarchy.undergrad_role_id,
            self.hierarchy.postgrad_role_id,
            self.hierarchy.mentor_role_id,
        ]

    def role_ids_needing_registration(self) -> list[int]:
        """
        The IDs of all roles that can be granted by the bot during the registration
        process.
        """
        return [
            # Hierarchy and user kind roles
            self.hierarchy.undergrad_role_id,
            self.hierarchy.postgrad_role_id,
            self.hierarchy.mentor_role_id,
            self.hierarchy.senior_mentor_role_id,
            self.hierarchy.honorary_mentor_role_id,
            self.hierarchy.faculty_role_id,
            # Pronoun roles
            self.pronouns.he_him_role_id,
            self.pronouns.she_her_role_id,
            self.pronouns.they_them_role_id,
            self.pronouns.xe_xem_role_id,
            self.pronouns.any_pronouns_role_id,
            self.pronouns.ask_pronouns_role_id,
            # Housing roles
            self.housing.jc_catered_role_id,
            self.housing.up_catered_role_id,
            self.housing.jc_self_catered_role_id,
            self.housing.up_self_catered_role_id,
            self.housing.private_house_role_id,
        ]
