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

pub mod commands;
use commands::*;

mod responses;

pub mod config;

#[derive(Clone)]
pub struct Norris(Arc<BotFramework>);

impl Norris {
    pub async fn new(config: NorrisConfig) -> BotResult<Self> {
        let framework = BotFramework::builder()
            .token(&config.bot_token)
            .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS)
            .options(FrameworkOptions {
                commands: vec![registration(), count(), nickname()],
                event_handler: |context, event, _, bot_data| {
                    Box::pin(events::event_handler(context, event, bot_data))
                },
                on_error: |err| Box::pin(async move { log::error!("{:?}", err) }), // TODO: Use a dedicated error handler
                ..FrameworkOptions::default()
            })
            .setup(move |context, _, framework| {
                Box::pin(setup_bot_data(context, framework, config))
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
    config: NorrisConfig,
) -> BotResult<BotData> {
    // Register slash commands in the guild
    let commands = builtins::create_application_commands(&framework.options().commands);
    config
        .guild_id
        .set_application_commands(&context.http, |guild_commands| {
            *guild_commands = commands;
            guild_commands
        })
        .await?;

    // Connect to and setup the database
    let database_pool = MySqlPoolOptions::new()
        .max_connections(25) // TODO: Find an appropriate max number of connections through testing
        .connect(&config.database_url)
        .await?;
    sqlx::query_file!("queries/create-table-users.sql")
        .execute(&database_pool)
        .await?;
    sqlx::query_file!("queries/create-table-registrations.sql")
        .execute(&database_pool)
        .await?;

    Ok(BotData {
        database_pool,
        guild_id: config.guild_id,
        channels: config.channels,
        roles: config.roles,
    })
}
