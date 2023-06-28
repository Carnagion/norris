from dataclasses import dataclass
from typing import Self

from serde import field, serde
from serde.toml import from_toml
from sqlalchemy import URL, make_url

from .channels import ChannelsConfig, PostgradChannelsConfig, UndergradChannelsConfig
from .roles import (
    HierarchyRolesConfig,
    Housing,
    HousingRolesConfig,
    PronounRolesConfig,
    Pronouns,
    RolesConfig,
)


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
