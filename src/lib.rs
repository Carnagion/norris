use std::sync::Arc;

use poise::{builtins, serenity_prelude as serenity, FrameworkOptions};

use serenity::*;

use sqlx::mysql::MySqlPoolOptions;

pub mod prelude;
use prelude::*;

mod data;
pub use data::*;

mod error;
pub use error::*;

pub mod model;

pub mod events;

#[derive(Clone)]
pub struct Norris(Arc<BotFramework>);

impl Norris {
    pub async fn new(
        token: impl Into<String>,
        guild_id: GuildId,
        database_url: String,
        arrival_channel_id: ChannelId,
        support_channel_id: ChannelId,
        log_channel_id: ChannelId,
    ) -> BotResult<Self> {
        let framework = BotFramework::builder()
            .token(token)
            .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS)
            .options(FrameworkOptions {
                commands: vec![], // TODO: Add commands once we actually have them
                event_handler: |context, event, _, bot_data| {
                    Box::pin(events::event_handler(context, event, bot_data))
                },
                on_error: |err| Box::pin(async move { log::error!("{}", err) }), // TODO: Use a dedicated error handler
                ..FrameworkOptions::default()
            })
            .setup(move |context, _, framework| {
                Box::pin(setup_bot_data(
                    context,
                    framework,
                    guild_id,
                    database_url,
                    arrival_channel_id,
                    support_channel_id,
                    log_channel_id,
                ))
            })
            .build()
            .await?;
        Ok(Norris(framework))
    }

    pub async fn start(self) -> BotResult<()> {
        self.0.start().await?;
        Ok(())
    }
}

async fn setup_bot_data(
    context: &Context,
    framework: &BotFramework,
    guild_id: GuildId,
    database_url: String,
    arrival_channel_id: ChannelId,
    support_channel_id: ChannelId,
    log_channel_id: ChannelId,
) -> BotResult<BotData> {
    // Register slash commands in the guild
    let commands = builtins::create_application_commands(&framework.options().commands);
    guild_id
        .set_application_commands(&context.http, |guild_commands| {
            *guild_commands = commands;
            guild_commands
        })
        .await?;

    // Connect to and setup the database
    let database_pool = MySqlPoolOptions::new()
        .max_connections(25) // TODO: Find an appropriate max number of connections through testing
        .connect(&database_url)
        .await?;
    sqlx::query_file!("queries/create-table-users.sql")
        .execute(&database_pool)
        .await?;
    sqlx::query_file!("queries/create-table-registrations.sql")
        .execute(&database_pool)
        .await?;

    Ok(BotData {
        database_pool,
        arrival_channel_id,
        support_channel_id,
        log_channel_id,
    })
}
