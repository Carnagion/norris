use std::{fs, io};

use anyhow::Result as AnyResult;

use tracing::level_filters::LevelFilter;

use tracing_appender::rolling;

use tracing_subscriber::{fmt::Layer, prelude::*};

use norris::prelude::*;

#[tokio::main]
async fn main() -> AnyResult<()> {
    // Deserialize config file
    let config_string = fs::read_to_string("norris.toml")?;
    let config = toml::from_str::<NorrisConfig>(&config_string)?;

    // Setup logging before continuing anything else
    let (file_logger, _file_guard) =
        tracing_appender::non_blocking(rolling::daily("", &config.log_path));
    let (stdout_logger, _stdout_guard) = tracing_appender::non_blocking(io::stdout());
    tracing_subscriber::registry()
        .with(
            Layer::new()
                .with_writer(stdout_logger)
                .with_filter(LevelFilter::INFO),
        )
        .with(
            Layer::new()
                .with_writer(file_logger)
                .with_filter(LevelFilter::WARN),
        )
        .try_init()?;

    // Create and start bot
    Norris::new(config).await?.start().await?;

    Ok(())
}
