mod note;

use tonic::transport::Server;
use proto::notes::note_manager_server::NoteManagerServer;
use note::NoteService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let note_service = NoteService::default();

    Server::builder()
        .add_service(NoteManagerServer::new(note_service))
        .serve(addr)
        .await?;

    Ok(())
}
