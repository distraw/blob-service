
use proto::notes::note_manager_client::NoteManagerClient;
use proto::notes::NoteRequest;
use tonic::Request;
use tonic::transport::channel::Channel;

use eyre::{Result, eyre};

#[derive(Clone)]
pub struct BlobClient {
    client: Option<NoteManagerClient<Channel>>,
    url: String,
}

impl BlobClient {
    pub fn new(url: String) -> Self {
        Self {
            client: None,
            url: url,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let client = NoteManagerClient::connect(self.url.clone()).await?;
        self.client = Some(client);
        Ok(())
    }

    pub async fn create_note(&mut self, content: String) -> Result<u32> {
        let client = self
            .client
            .as_mut()
            .ok_or_else(||eyre!("client is not connected to blob service"))?;

        let request = Request::new(NoteRequest {
            content: content,
        });

        let response = client.create_note(request).await?;
        Ok(response.into_inner().id)
    }
}