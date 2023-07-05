use poise::serenity_prelude as serenity;

use serenity::*;

use sqlx::MySqlPool;

use crate::prelude::*;

/// Data held and used by [`Norris`] during its operation.
#[derive(Clone, Debug)]
pub struct BotData {
    /// A connection pool to the MySQL database storing [`VerifiedUser`]s and [`Registration`] information.
    pub database_pool: MySqlPool,
    /// The ID of the [`Guild`] where the bot is operating.
    pub guild_id: GuildId,
    /// [`Channel`]-related configuration data.
    pub channels: ChannelsConfig,
    /// [`Role`]-related configuration data.
    pub roles: RolesConfig,
}
