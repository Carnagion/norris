use std::sync::Arc;

use poise::{builtins, serenity_prelude as serenity, FrameworkOptions};

use serenity::*;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

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

    // Setup database and tables
    let database_pool = setup_database(&config.database_url).await?;

    Ok(BotData {
        database_pool,
        guild_id: config.guild_id,
        channels: config.channels,
        roles: config.roles,
    })
}

async fn setup_database(database_url: &str) -> BotResult<MySqlPool> {
    // Connect to the database
    let database_pool = MySqlPoolOptions::new()
        .max_connections(25) // TODO: Find an appropriate max number of connections through testing
        .connect(database_url)
        .await?;

    // Setup users table
    sqlx::query!(
        r#"create table if not exists users (
            id bigint unsigned not null auto_increment primary key,
            name varchar(1024) not null,
            kind enum(
                "UNDERGRAD",
                "POSTGRAD",
                "MENTOR",
                "SENIOR_MENTOR",
                "HONORARY_MENTOR",
                "FACULTY"
            ) not null,
            registered_user_id bigint unsigned null
        )"#
    )
    .execute(&database_pool)
    .await?;

    // Setup registrations table
    sqlx::query!(
        r#"create table if not exists registrations (
            user_id bigint unsigned not null primary key,
            status enum(
                "UNREGISTERED",
                "STARTED",
                "NAME_ENTERED",
                "KIND_FOUND",
                "VERIFIED",
                "PRONOUNS_PICKED",
                "REGISTERED",
                "FAILED"
            ) not null default "UNREGISTERED",
            name varchar(1024) null,
            kind enum(
                "UNDERGRAD",
                "POSTGRAD",
                "MENTOR",
                "SENIOR_MENTOR",
                "HONORARY_MENTOR",
                "FACULTY"
            ) null
        )"#
    )
    .execute(&database_pool)
    .await?;

    Ok(database_pool)
}
