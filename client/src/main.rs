// https://www.youtube.com/watch?v=JkSa-qA2jnY&t

use proto::notes::note_manager_client::NoteManagerClient;
use proto::notes::NoteRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut client = NoteManagerClient::connect(
        "http://[::1]:50051"
    ).await?;

    let request = tonic::Request::new(
        NoteRequest {
            content: "This is my test note!".to_owned(),
        }
    );

    let response = client.create_note(request).await?;
    println!("Response: {:?}", response);

    Ok(())
}