use std::{fs::File, io::Error as IoError, path::PathBuf};

use dotenvy::Error as DotEnvError;

use envy::Error as EnvError;

use log::SetLoggerError;

use poise::{
    builtins,
    serenity_prelude::{Context, GatewayIntents, GuildId, SerenityError},
    FrameworkOptions,
};

use serde::Deserialize;

use simplelog::{
    ColorChoice, CombinedLogger, Config as LoggerConfig, LevelFilter, TermLogger, TerminalMode,
    WriteLogger,
};

use sqlx::mysql::MySqlPoolOptions;

use thiserror::Error;

use norris::{BotData, BotError, BotFramework};

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    // Load .env file
    dotenvy::dotenv()?;

    // Parse configuration values from the environment
    let Config {
        bot_token,
        guild_id,
        database_url,
        log_path,
    } = envy::from_env::<Config>()?;

    // Setup logger
    setup_logger(log_path)?; // NOTE: This should only be called once, and before starting the bot

    // Build and start bot
    BotFramework::builder()
        .token(bot_token)
        .intents(GatewayIntents::non_privileged())
        .options(FrameworkOptions {
            commands: vec![],
            ..FrameworkOptions::default()
        })
        .setup(move |context, _, framework| {
            Box::pin(setup_bot(context, framework, guild_id, database_url))
        })
        .build()
        .await?
        .start()
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Config {
    bot_token: String,
    guild_id: GuildId,
    database_url: String,
    log_path: PathBuf,
}

#[derive(Debug, Error)]
enum StartupError {
    #[error("{}", .0)]
    Io(#[from] IoError),
    #[error("{}", .0)]
    Discord(#[from] SerenityError),
    #[error("{}", .0)]
    DotEnv(#[from] DotEnvError),
    #[error("{}", .0)]
    Env(#[from] EnvError),
    #[error("{}", .0)]
    Logger(#[from] SetLoggerError),
}

fn setup_logger(log_path: PathBuf) -> Result<(), StartupError> {
    CombinedLogger::init(vec![
        // Log most events to stdout
        TermLogger::new(
            LevelFilter::Info,
            LoggerConfig::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // Log warnings and errors to a log file
        WriteLogger::new(
            LevelFilter::Warn,
            LoggerConfig::default(),
            File::create(log_path)?,
        ),
    ])?;
    Ok(())
}

async fn setup_bot(
    context: &Context,
    framework: &BotFramework,
    guild_id: GuildId,
    database_url: String,
) -> Result<BotData, BotError> {
    // Register slash commands in the guild
    let commands = builtins::create_application_commands(&framework.options().commands);
    guild_id
        .set_application_commands(&context.http, |guild_commands| {
            *guild_commands = commands;
            guild_commands
        })
        .await?;

    // Create a pool of connections to the database
    let database_pool = MySqlPoolOptions::new()
        .max_connections(25) // TODO: Find the right max number of connections through testing
        .connect(&database_url)
        .await?;

    sqlx::query_file!("queries/create-table-users.sql")
        .execute(&database_pool)
        .await?;
    sqlx::query_file!("queries/create-table-registrations.sql")
        .execute(&database_pool)
        .await?;

    Ok(BotData { database_pool })
}
