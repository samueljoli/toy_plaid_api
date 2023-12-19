use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub db_max_connections: u32,
    pub database_url: String,
}

impl Settings {
    pub fn new(directory: &String) -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        //Hierarchial/Layered config composition.
        let s = Config::builder()
            // Load from defaults configs.
            .add_source(File::with_name(&format!("{}/default.toml", directory)))
            // Load from "env" based configs.
            .add_source(
                File::with_name(&format!("{}/{}.toml", directory, run_mode)).required(false),
            )
            // Load from ENVIRONMENT
            .add_source(Environment::default())
            .build()?;

        // Deserialize and freeze configs as is.
        s.try_deserialize()
    }
}
