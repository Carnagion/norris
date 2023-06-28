from dataclasses import dataclass

from serde import serde


@serde(rename_all="kebabcase")
@dataclass
class UndergradChannelsConfig:
    main_channel_id: int


@serde(rename_all="kebabcase")
@dataclass
class PostgradChannelsConfig:
    main_channel_id: int
    common_channel_id: int


@serde(rename_all="kebabcase")
@dataclass
class ChannelsConfig:
    arrival_channel_id: int
    support_channel_id: int
    log_channel_id: int
    undergrad: UndergradChannelsConfig
    postgrad: PostgradChannelsConfig
