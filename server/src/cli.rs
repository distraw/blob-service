// https://github.com/distributed-lab/op_rand/blob/main/apps/cli/src/actions/mod.rs

use clap::Parser;
use clap_verbosity::Verbosity;

use tracing_log::AsTrace;

use tonic::transport::Server;
use proto::notes::note_manager_server::NoteManagerServer;

use crate::note::NoteService;

use ui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: Verbosity,
}

impl Cli {
    pub async fn run(self) -> eyre::Result<()> {
        println!("running CLI...");
        tracing_subscriber::fmt()
            .with_max_level(self.verbosity.log_level_filter().as_trace())
            .init();

        let addr = "[::1]:50051".parse()?;
        let note_service = NoteService::default();

        println!("{}", ui::header("🎯 BLOB-SERVICE 🎯"));

        Server::builder()
            .add_service(NoteManagerServer::new(note_service))
            .serve(addr)
            .await?;

        Ok(())
    }
}