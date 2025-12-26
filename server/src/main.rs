mod note;
mod config;
mod cli;
mod context;
mod db;

use clap::Parser;
use cli::Cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    Cli::parse().run().await
}
