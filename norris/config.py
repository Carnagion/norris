from dataclasses import dataclass
from typing import Self

from serde import field, serde
from serde.toml import from_toml
from sqlalchemy import URL, make_url


@serde(rename_all="kebabcase")
@dataclass
class ChannelsConfig:
    arrival_channel_id: int
    support_channel_id: int
    chat_channel_id: int
    log_channel_id: int


@serde(rename_all="kebabcase")
@dataclass
class HierarchyRolesConfig:
    undergrad_role_id: int
    postgrad_role_id: int
    mentor_role_id: int
    senior_mentor_role_id: int
    honorary_mentor_role_id: int
    faculty_role_id: int


@serde(rename_all="kebabcase")
@dataclass
class PronounRolesConfig:
    he_him_role_id: int
    she_her_role_id: int
    they_them_role_id: int
    xe_xem_role_id: int
    any_pronouns_role_id: int
    ask_pronouns_role_id: int


@serde(rename_all="kebabcase")
@dataclass
class HousingRolesConfig:
    jc_catered_role_id: int
    jc_self_catered_role_id: int
    up_catered_role_id: int
    up_self_catered_role_id: int
    private_house_role_id: int


@serde(rename_all="kebabcase")
@dataclass
class RolesConfig:
    hierarchy: HierarchyRolesConfig
    pronouns: PronounRolesConfig
    housing: HousingRolesConfig


@serde(rename_all="kebabcase")
@dataclass
class NorrisConfig:
    bot_token: str
    guild_id: int
    log_path: str  # TODO: use when logging is setup
    channels: ChannelsConfig
    roles: RolesConfig
    database_url: URL = field(  # NOQA: RUF009 - this is how serde is meant to be used
        deserializer=lambda url: make_url(url).set(drivername="mysql+asyncmy"),
    )

    @staticmethod
    def from_toml(toml: str) -> Self:
        return from_toml(NorrisConfig, toml)
