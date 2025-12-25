// https://github.com/distributed-lab/op_rand/blob/main/apps/cli/src/actions/mod.rs

use std::path::{PathBuf};

use clap::Parser;
use clap_verbosity::Verbosity;

use tracing_log::AsTrace;

use tonic::transport::Server;
use proto::notes::note_manager_server::NoteManagerServer;

use crate::{note::NoteService, context::Context};

use ui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: Verbosity,

    #[clap(short, long, default_value = "config/server.local.toml")]
    pub config: PathBuf,
}

impl Cli {
    pub async fn run(self) -> eyre::Result<()> {
        tracing_subscriber::fmt()
            .with_max_level(self.verbosity.log_level_filter().as_trace())
            .init();

        let mut context = Context::new(self.config);

        let addr = context.config()?.addr;
        let note_service = NoteService::default();

        println!("{}", ui::header("🎯 BLOB-SERVICE 🎯"));

        Server::builder()
            .add_service(NoteManagerServer::new(note_service))
            .serve(addr.parse()?)
            .await?;

        Ok(())
    }
}