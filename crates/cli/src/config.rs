use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    /// URL to auth service
    pub auth_url: String,
    /// Application id for auth service
    pub auth_app_id: String,
    /// Path of static files to serve
    #[serde(default = "default_static_path")]
    pub static_path: PathBuf,
    /// Directory to load facts from
    pub facts_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("ANNAPURNA"))
            .build()?;

        config.try_deserialize()
    }
}

fn default_static_path() -> PathBuf {
    PathBuf::from(".")
}
