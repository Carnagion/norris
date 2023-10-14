use std::{fs, io};

use anyhow::Result as AnyResult;

use tokio::{select, signal};

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

    tracing::warn!("Starting up");

    let norris = Norris::new(config).await?;
    select! {
        // Listen for interruption and shutdown gracefully
        _ = signal::ctrl_c() => {
            tracing::warn!("Shutting down due to interruption");
            Ok(())
        },
        // Shutdown if there was an error during runtime
        Err(err) = norris.start() => {
            tracing::warn!("Shutting down due to error");
            tracing::error!("{}", err);
            Err(err)
        },
        // NOTE: This should never be reached since the bot runs forever but it is included here for completeness anyways
        else => Ok(()),
    }?;

    Ok(())
}
