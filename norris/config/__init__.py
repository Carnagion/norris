"""
Configuration types.
"""

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
    """
    Secrets and other configuration data loaded through a `norris.toml` configuration
    file.
    """

    bot_token: str
    """
    The bot's Discord token.
    """

    guild_id: int
    """
    The ID of the guild where the bot is running.
    """

    log_path: str  # TODO: use when logging is setup
    """
    A path at which a log file will be created and written to for the duration of the 
    bot's operation.
    """

    channels: ChannelsConfig
    """
    Channel-related configuration data.
    """

    roles: RolesConfig
    """
    Role-related configuration data.
    """

    database_url: URL = field(  # NOQA: RUF009 - this is how serde is meant to be used
        deserializer=lambda url: make_url(url).set(drivername="mysql+asyncmy"),
    )
    """
    A MySQL database connection URL in the format `mysql://user:password@host/database`.
    """

    @staticmethod
    def from_toml(toml: str) -> Self:
        """
        Load a `NorrisConfig` from a TOML file.
        """
        return from_toml(NorrisConfig, toml)
