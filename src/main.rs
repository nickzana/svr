use config::Config;
use futures::future::join_all;
use std::sync::Arc;
use thiserror::Error;
use tokio::{
    fs,
    io::{self, AsyncReadExt},
};

pub mod config;
mod flags;

const DEFAULT_CONFIG_PATH: &str = "./config.toml";

#[derive(Debug, Error)]
pub enum Error {
    #[error("IoError: {0:#?}")]
    IoError(#[from] io::Error),
    #[error("Invalid config: {0}")]
    InvalidConfig(#[from] toml::de::Error),
    #[error("Invalid command line flags: {0:#?}")]
    Arguments(#[from] xflags::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let flags = flags::Svr::from_env()?;

    let config_path = flags.config.unwrap_or_else(|| DEFAULT_CONFIG_PATH.into());

    let mut config = String::new();
    fs::File::open(&config_path)
        .await?
        .read_to_string(&mut config)
        .await?;

    let config: Arc<Config> = Arc::new(toml::from_str(&config)?);

    // Spawn a task for each stream to watch it
    // This config.streams.clone seems unnecessary since we only need an immutable
    // reference
    join_all(config.streams.clone().into_iter().map(|stream| {
        let config = Arc::clone(&config);
        tokio::spawn(async move {
            stream.watch(&config).await.unwrap();
        })
    }))
    .await;

    Ok(())
}
