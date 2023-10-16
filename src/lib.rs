//! A registration bot for the [University of Nottingham](https://www.nottingham.ac.uk) Computer Science Discord server.

#![warn(missing_docs)]

use std::{io, sync::Arc};

use poise::{builtins, serenity_prelude as serenity, FrameworkOptions};

use serenity::*;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use tokio::signal;

pub mod prelude;
use prelude::*;

pub mod config;

mod data;
pub use data::*;

mod error;
pub use error::BotError;

pub mod model;

pub mod events;

pub mod commands;

pub mod responses;

/// An abstraction over a [`BotFramework`], allowing for easy instantiation with all the relevant commands, handlers, and configuration.
#[derive(Clone)]
pub struct Norris(Arc<BotFramework>);

impl Norris {
    /// Creates a new instance of [`Norris`] with the provided configuration.
    ///
    /// # Errors
    ///
    /// Fails if building the inner [`BotFramework`] fails.
    /// See [`BotFramework::builder`].
    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn new(config: NorrisConfig) -> BotResult<Self> {
        let framework = BotFramework::builder()
            .token(&config.bot_token)
            .intents(GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS)
            .options(FrameworkOptions {
                commands: vec![
                    commands::registration(),
                    commands::count(),
                    commands::nickname(),
                ],
                event_handler: |context, event, _, bot_data| {
                    Box::pin(events::event_handler(context, event, bot_data))
                },
                on_error: |err| Box::pin(error::report_framework_error(err)),
                ..FrameworkOptions::default()
            })
            .setup(move |context, _, framework| {
                Box::pin(setup_bot_data(context, framework, config))
            })
            .build()
            .await?;
        Ok(Norris(framework))
    }

    /// Starts the bot and keeps it running in an asynchronous loop.
    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn start(self) -> BotResult<()> {
        self.handle_shutdown();
        self.0.start().await?;
        Ok(())
    }

    fn handle_shutdown(&self) {
        let shard_manager = Arc::clone(self.0.shard_manager());
        let http = Arc::clone(&self.0.client().cache_and_http.http);
        let framework = Arc::clone(&self.0);

        tokio::spawn(async move {
            // Wait for a shutdown signal
            let shutdown = shutdown_signal().await;
            if let Err(err) = shutdown {
                tracing::error!("Could not setup shutdown signal: {}", err);
            }

            // Send message before shutting down
            let config = framework.user_data().await;
            let sent = config
                .channels
                .log_channel_id
                .send_message(http, |message| {
                    message
                        .content(format!("<&{}>", config.roles.hierarchy.admin_role_id))
                        .embed(responses::embeds::logs::shutting_down())
                })
                .await;
            if let Err(err) = sent {
                tracing::error!("Could not send shutdown alert: {}", err);
            }

            // Tell the shard to shut down
            shard_manager.lock().await.shutdown_all().await;
        });
    }
}

#[tracing::instrument(skip_all, err(Debug))]
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

    // Get rid of any global application commands that might have been set accidentally during testing
    Command::set_global_application_commands(context.http(), |commands| commands).await?;

    // Setup database and tables
    let database_pool = setup_database(&config.database_url).await?;

    Ok(BotData {
        database_pool,
        guild_id: config.guild_id,
        channels: config.channels,
        roles: config.roles,
    })
}

#[tracing::instrument(skip_all, err(Debug))]
async fn setup_database(database_url: &str) -> BotResult<MySqlPool> {
    // Connect to the database
    let database_pool = MySqlPoolOptions::new()
        .max_connections(150) // NOTE: Limit taken from https://docs.rs/sqlx/latest/sqlx/struct.Pool.html#2-connection-limits-mysql-mssql-postgres
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

async fn shutdown_signal() -> io::Result<()> {
    #[cfg(windows)]
    {
        use signal::windows;

        // Handle Ctrl-C, Ctrl-Break, and Ctrl-Close on Windows systems
        let mut ctrl_c = windows::ctrl_c()?;
        let mut ctrl_break = windows::ctrl_break()?;
        let mut ctrl_close = windows::ctrl_close()?;
        tokio::select! {
            _ = ctrl_c.recv() => {},
            _ = ctrl_break.recv() => {},
            _ = ctrl_close.recv() => {},
        }
    }

    #[cfg(unix)]
    {
        use signal::unix::{self, SignalKind};

        // Handle SIGINT, SIGTERM, and SIGQUIT on Unix systems
        let mut sigint = unix::signal(SignalKind::interrupt())?;
        let mut sigterm = unix::signal(SignalKind::terminate())?;
        let mut sigquit = unix::signal(SignalKind::quit())?;
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
            _ = sigquit.recv() => {},
        }
    }

    Ok(())
}
