use poise::serenity_prelude as serenity;

use serde::{Deserialize, Serialize};

use serenity::*;

/// [`Channel`]-related configuration data and mappings.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChannelsConfig {
    /// The ID of the channel where arrival messages are posted by Discord.
    pub arrival_channel_id: ChannelId,
    /// The ID of the channel where registration support issues are handled.
    pub support_channel_id: ChannelId,
    /// The ID of the channel where registration processes are logged.
    pub log_channel_id: ChannelId,
    /// The ID of the channel where nickname requests are made.
    pub nickname_channel_id: ChannelId,
    /// Undergraduate-only channel configuration data.
    pub undergrad: UndergradChannelsConfig,
    /// Postgraduate-only channel configuration data.
    pub postgrad: PostgradChannelsConfig,
}

impl ChannelsConfig {
    /// The IDs of all postgraduate-only [`Channel`]s.
    pub fn postgrad_channel_ids(&self) -> [ChannelId; 2] {
        [
            self.postgrad.main_channel_id,
            self.postgrad.common_channel_id,
        ]
    }
}

/// Configuration data for undergraduate-only [`Channel`]s.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UndergradChannelsConfig {
    /// The main undergraduate channel accessible to both staff and students.
    pub main_channel_id: ChannelId,
}

/// Configuration data for postgraduate-only [`Channel`]s.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PostgradChannelsConfig {
    /// The main postgraduate channel accessible to both staff and students.
    pub main_channel_id: ChannelId,
    /// The secondary channel accessible to only students.
    pub common_channel_id: ChannelId,
}
