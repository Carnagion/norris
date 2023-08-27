//! Configuration types.

use std::path::PathBuf;

use poise::serenity_prelude as serenity;

use serde::{Deserialize, Serialize};

use serenity::*;

mod channels;
pub use channels::*;

mod roles;
pub use roles::*;

/// Secrets and other configuration data loaded through a `norris.toml` configuration file.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NorrisConfig {
    /// The bot's Discord token.
    pub bot_token: String,
    /// A MySQL database connection URL in the format `mysql://user:password@host/database`.
    pub database_url: String,
    /// The ID of the [`Guild`] where the bot is running.
    pub guild_id: GuildId,
    /// A path at which a log file will be created and written to for the duration of the bot's operation.
    pub log_path: PathBuf,
    /// [`Channel`]-related configuration data.
    pub channels: ChannelsConfig,
    /// [`Role`]-related configuration data.
    pub roles: RolesConfig,
}
