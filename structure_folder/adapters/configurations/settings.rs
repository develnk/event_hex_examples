use std::{env, sync::{Arc, OnceLock}};

use config::{Config, ConfigError, File};
use serde::Deserialize;

static GLOBAL_SETTINGS: OnceLock<Arc<AppSettings>> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct DbCredentials {
    pub url: String,
    pub username: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings {
    pub database: DbCredentials,
    pub logging_level: String,
}

impl AppSettings {
    pub fn init() -> Result<(), ConfigError> {
        let run_mod = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{run_mod}")).required(false))
            .build()?;

        let app_settings: AppSettings = config.try_deserialize()?;
        let _ = GLOBAL_SETTINGS.set(Arc::new(app_settings.to_owned()));
        println!("Settings loaded successfully:");
        println!("{:#?}", app_settings);
        Ok(())
    }
}

pub fn get_app_settings() -> Arc<AppSettings> {
    Arc::clone(GLOBAL_SETTINGS.get().unwrap())
}
