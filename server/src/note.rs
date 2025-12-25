// https://www.youtube.com/watch?v=JkSa-qA2jnY&t

use tonic::{Request, Response, Status};
use proto::notes::{NoteRequest, NoteResponse};
use proto::notes::note_manager_server::NoteManager;

use ui;

#[derive(Debug, Default)]
pub struct NoteService {}

#[tonic::async_trait]
impl NoteManager for NoteService {
    async fn create_note(&self, request: Request<NoteRequest>) -> Result<Response<NoteResponse>, Status> {
        let note = request.into_inner();
        println!("{}", ui::success_footer("New note arrived"));
        println!("Content: {:?}", note.content);

        let reply = NoteResponse {
            id: 123,
        };

        Ok(Response::new(reply))
    }
}