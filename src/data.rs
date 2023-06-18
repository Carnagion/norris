use sqlx::MySqlPool;

#[derive(Clone, Debug)]
pub struct BotData {
    pub database_pool: MySqlPool,
}
