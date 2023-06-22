use poise::serenity_prelude as serenity;

use serenity::*;

use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct BotData {
    pub database_pool: MySqlPool,
    pub arrival_channel_id: ChannelId,
    pub support_channel_id: ChannelId,
    pub log_channel_id: ChannelId,
}
