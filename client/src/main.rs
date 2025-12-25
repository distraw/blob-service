// https://github.com/distributed-lab/op_rand/blob/main/apps/cli/src/main.rs

mod actions;
mod config;
mod blob;
mod context;

use clap::Parser;

use crate::actions::Cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()>{
    color_eyre::install()?;
    Cli::parse().run().await
}