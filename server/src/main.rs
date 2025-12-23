// https://www.youtube.com/watch?v=JkSa-qA2jnY&t

use tonic::{transport::Server, Request, Response, Status};

use proto::notes::note_manager_server::{NoteManager, NoteManagerServer};
use proto::notes::{NoteRequest, NoteResponse};

#[derive(Debug, Default)]
pub struct NoteService {}

#[tonic::async_trait]
impl NoteManager for NoteService {
    async fn create_note(&self, request: Request<NoteRequest>) -> Result<Response<NoteResponse>, Status> {
        let note = request.into_inner();
        println!("Got a note: {:?}", note.content);

        let reply = NoteResponse {
            id: 123,
        };

        Ok(Response::new(reply))
    }
}

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
