use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::{self, EnvFilter};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub network: String,
    pub contract: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::try_from_default_env().unwrap())
            .init();

        info!("getting configurations...");
        //make an empty config variable using Config struct
        let mut c = config::Config::new();
        //merge the empty config variable with the .env variable
        c.merge(config::Environment::default())?;

        info! {"getting configurations : OK"}

        //convert the config variable into the Config struct
        return c
            .try_into()
            .context("loading configurations from environment...");
    }

    pub fn from_env_without_tracing() -> Result<Config> {
        dotenv().ok();

        //make an empty config variable using Config struct
        let mut c = config::Config::new();
        //merge the empty config variable with the .env variable
        c.merge(config::Environment::default())?;

        //convert the config variable into the Config struct
        return c
            .try_into()
            .context("loading configurations from environment...");
    }
}
