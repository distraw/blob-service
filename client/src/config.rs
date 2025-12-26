// https://github.com/distributed-lab/op_rand/blob/main/apps/cli/src/config.rs

use std::path::PathBuf;
use color_eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
    pub blob_url: String,
}

impl Config {
    pub fn from_path(path: PathBuf) -> eyre::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::from(path))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}