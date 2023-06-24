use sqlx::MySqlPool;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct BotData {
    pub database_pool: MySqlPool,
    pub channels: ChannelsConfig,
    pub roles: RolesConfig,
}
