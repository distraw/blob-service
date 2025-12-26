// https://www.youtube.com/watch?v=JkSa-qA2jnY&t

use tonic::{Request, Response, Status};
use proto::notes::{Note, NoteId};
use proto::notes::note_manager_server::NoteManager;

use crate::db::Database;
use ui;

#[derive(Debug, Default)]
pub struct NoteService {
   pub db: Database,
}

#[tonic::async_trait]
impl NoteManager for NoteService {
    async fn create_note(&self, request: Request<Note>) -> Result<Response<NoteId>, Status> {
        let note = request.into_inner();
        println!("{}", ui::success_footer("New note arrived"));
        println!("Content: {:?}", note.content);

        let id = self
            .db
            .insert(note.content)
            .await
            .map_err(|e| {
                Status::internal(format!("DB insert failed: {}", e))
        })?;

        let reply = NoteId {
            id: id,
        };

        Ok(Response::new(reply))
    }

    async fn get_note(&self, request: Request<NoteId>) -> Result<Response<Note>, Status> {
        let noteid = request.into_inner();
        println!("{}", ui::success_footer("New request for note arrived"));
        println!("ID: {:?}", noteid.id);

        let content = self
            .db
            .fetch(noteid.id)
            .await
            .map_err(|e| {
                Status::internal(format!("DB fetch failed: {}", e))
            })?;

        let reply = Note {
            content: content,
        };
        
        Ok(Response::new(reply))
    }
}