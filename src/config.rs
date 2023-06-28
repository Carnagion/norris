use std::path::PathBuf;

use poise::serenity_prelude as serenity;

use serde::{Deserialize, Serialize};

use serenity::*;

mod channels;
pub use channels::*;

mod roles;
pub use roles::*;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NorrisConfig {
    pub(crate) bot_token: String,
    pub(crate) database_url: String,
    pub guild_id: GuildId,
    pub log_path: PathBuf,
    pub channels: ChannelsConfig,
    pub roles: RolesConfig,
}
