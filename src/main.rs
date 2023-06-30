use std::{
    fs::{self, File},
    path::Path,
};

use anyhow::Result as AnyResult;

use simplelog::{
    ColorChoice, CombinedLogger, Config as LoggerConfig, LevelFilter, TermLogger, TerminalMode,
    WriteLogger,
};

use norris::prelude::*;

#[tokio::main]
async fn main() -> AnyResult<()> {
    // Deserialize config file
    let config_string = fs::read_to_string("norris.toml")?;
    let config = toml::from_str::<NorrisConfig>(&config_string)?;

    // Setup logging before continuing anything else
    setup_logger(&config.log_path).await?;

    // Create and start bot
    Norris::new(config).await?.start().await?;

    Ok(())
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
