// https://github.com/distributed-lab/op_rand/blob/main/apps/cli/src/actions/mod.rs

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_verbosity::Verbosity;
use tracing_log::AsTrace;

use crate::{
    actions::{
        create_note::CreateNoteArgs,
        get_note::GetNoteArgs,
    },
    context::Context,
};

mod create_note;
mod get_note;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: Verbosity,

    #[command(subcommand)]
    pub command: Commands,

    #[clap(short, long, default_value = "config/client.local.toml")]
    pub config: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // Create a note
    CreateNote(CreateNoteArgs),
    GetNote(GetNoteArgs),
}

impl Cli {
    pub async fn run(self) -> eyre::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(self.verbosity.log_level_filter().as_trace())
            .init();

        let context = Context::new(self.config);
        execute_command(self.command, context).await
    }
}

async fn execute_command(command: Commands, context: Context) -> eyre::Result<()> {
    use Commands as Cmd;
    match command {
        Cmd::CreateNote(cmd) => create_note::run(cmd, context).await,
        Cmd::GetNote(cmd) => get_note::run(cmd, context).await,
    }
}