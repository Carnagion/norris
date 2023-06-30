from dataclasses import dataclass
from enum import Enum

from serde import serde

from ..model import VerifiedUserKind


@serde(rename_all="kebabcase")
@dataclass
class HierarchyRolesConfig:
    undergrad_role_id: int
    postgrad_role_id: int
    mentor_role_id: int
    senior_mentor_role_id: int
    honorary_mentor_role_id: int
    faculty_role_id: int

    def role_id(self, kind: VerifiedUserKind) -> int:
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
    HE_HIM = "he_him"
    SHE_HER = "she_her"
    THEY_THEM = "they_them"
    XE_XEM = "xe_xem"
    ANY = "any"
    ASK = "ask"


@serde(rename_all="kebabcase")
@dataclass
class PronounRolesConfig:
    he_him_role_id: int
    she_her_role_id: int
    they_them_role_id: int
    xe_xem_role_id: int
    any_pronouns_role_id: int
    ask_pronouns_role_id: int

    def role_id(self, pronouns: Pronouns) -> int:
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
    JC_CATERED = "JC_CATERED"
    JC_SELF_CATERED = "JC_SELF_CATERED"
    UP_CATERED = "UP_CATERED"
    UP_SELF_CATERED = "UP_SELF_CATERED"
    PRIVATE_HOUSE = "PRIVATE_HOUSE"


@serde(rename_all="kebabcase")
@dataclass
class HousingRolesConfig:
    jc_catered_role_id: int
    jc_self_catered_role_id: int
    up_catered_role_id: int
    up_self_catered_role_id: int
    private_house_role_id: int

    def role_id(self, housing: Housing) -> int:
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
    hierarchy: HierarchyRolesConfig
    pronouns: PronounRolesConfig
    housing: HousingRolesConfig

    def nukeable_role_ids(self) -> list[int]:
        return [
            self.hierarchy.undergrad_role_id,
            self.hierarchy.postgrad_role_id,
            self.hierarchy.mentor_role_id,
        ]
