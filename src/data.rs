use poise::serenity_prelude as serenity;

use serenity::*;

use sqlx::MySqlPool;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct BotData {
    pub database_pool: MySqlPool,
    pub guild_id: GuildId,
    pub channels: ChannelsConfig,
    pub roles: RolesConfig,
}
