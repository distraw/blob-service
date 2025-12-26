use std::path::{PathBuf};
use color_eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
    pub addr: String,
    pub db_url: String, 
}

impl Config {
    pub fn from_path(path: PathBuf) -> eyre::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::from(path))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}