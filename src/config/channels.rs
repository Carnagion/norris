use poise::serenity_prelude as serenity;

use serde::{Deserialize, Serialize};

use serenity::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChannelsConfig {
    pub arrival_channel_id: ChannelId,
    pub support_channel_id: ChannelId,
    pub log_channel_id: ChannelId,
    pub undergrad: UndergradChannelsConfig,
    pub postgrad: PostgradChannelsConfig,
}

impl ChannelsConfig {
    pub fn postgrad_channel_ids(&self) -> [ChannelId; 2] {
        [
            self.postgrad.main_channel_id,
            self.postgrad.common_channel_id,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UndergradChannelsConfig {
    pub main_channel_id: ChannelId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostgradChannelsConfig {
    pub main_channel_id: ChannelId,
    pub common_channel_id: ChannelId,
}
