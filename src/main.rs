use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::Result as AnyResult;

use poise::serenity_prelude as serenity;

use serde::Deserialize;

use serenity::*;

use simplelog::{
    ColorChoice, CombinedLogger, Config as LoggerConfig, LevelFilter, TermLogger, TerminalMode,
    WriteLogger,
};

use norris::Norris;

#[tokio::main]
async fn main() -> AnyResult<()> {
    // Load .env files
    dotenvy::dotenv()?;

    // Parse environment variables
    let BotArgs {
        bot_token,
        guild_id,
        database_url,
        arrival_channel_id,
        support_channel_id,
        log_channel_id,
        log_path,
    } = envy::from_env()?;

    // Setup logging before continuing anything else
    setup_logger(&log_path).await?;

    Norris::new(
        bot_token,
        guild_id,
        database_url,
        arrival_channel_id,
        support_channel_id,
        log_channel_id,
    )
    .await?
    .start()
    .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct BotArgs {
    bot_token: String,
    guild_id: GuildId,
    database_url: String, // NOTE: This is also used by sqlx to check queries at compile time
    arrival_channel_id: ChannelId,
    support_channel_id: ChannelId,
    log_channel_id: ChannelId,
    log_path: PathBuf,
}

async fn setup_logger(log_path: impl AsRef<Path>) -> AnyResult<()> {
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
