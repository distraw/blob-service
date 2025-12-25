use std::path::PathBuf;

use color_eyre::{eyre, eyre::Context as _};

use crate::config::Config;

pub struct Context {
    config_path: PathBuf,

    config: Option<Config>,
}

impl Context {
    pub fn new(config: PathBuf) -> Self {
        Self {
            config_path: config,
            config: None,
        }
    }

    pub fn config(&mut self) -> eyre::Result<Config> {
        if let Some(config) = &self.config {
            return Ok(config.clone());
        }

        let cfg = Config::from_path(self.config_path.clone()).wrap_err("Failed to load config")?;

        self.config = Some(cfg.clone());

        Ok(cfg)
    }
}