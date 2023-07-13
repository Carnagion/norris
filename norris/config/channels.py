"""
Channel-related configuration types.
"""

from dataclasses import dataclass

from serde import serde


@serde(rename_all="kebabcase")
@dataclass
class UndergradChannelsConfig:
    """
    Configuration data for undergraduate-only channels.
    """

    main_channel_id: int
    """
    The main undergraduate channel accessible to both staff and students.
    """


@serde(rename_all="kebabcase")
@dataclass
class PostgradChannelsConfig:
    """
    Configuration data for postgraduate-only channels.
    """

    main_channel_id: int
    """
    The main postgraduate channel accessible to both staff and students.
    """

    common_channel_id: int
    """
    The secondary channel accessible to only students.
    """


@serde(rename_all="kebabcase")
@dataclass
class ChannelsConfig:
    """
    Channel-related configuration data and mappings.
    """

    arrival_channel_id: int
    """
    The ID of the channel where arrival messages are posted by Discord.
    """

    support_channel_id: int
    """
    The ID of the channel where registration support issues are handled.
    """

    log_channel_id: int
    """
    The ID of the channel where registration processes are logged.
    """

    nickname_channel_id: int
    """
    The ID of the channel where nickname requests are made.
    """

    undergrad: UndergradChannelsConfig
    """
    Undergraduate-only channel configuration data.
    """

    postgrad: PostgradChannelsConfig
    """
    Postgraduate-only channel configuration data.
    """
