// https://github.com/distributed-lab/op_rand/blob/main/apps/cli/src/actions/create_challenge/mod.rs

use clap::Args;

use color_eyre::{
    eyre
};
use eyre::Context as _;

use crate::{
    context::Context,
    blob::BlobClient,
};

use ui;

#[derive(Args, Debug)]
pub struct CreateNoteArgs {
    pub content: String,
}

pub async fn run(
    CreateNoteArgs {
        content,
    }: CreateNoteArgs,
    mut ctx: Context,
) -> eyre::Result<()> {
    println!(
        "{}",
        ui::header("🎯 CREATING NOTE 🎯")
    );

    let url = ctx.config()?.blob_url;
    println!("Content: {}\nBlob-url: {}", content, url);

    println!(
        "{}",
        ui::section_header("Connecting...")
    );
    let mut client = BlobClient::new(url);
    client.connect().await?;

    println!(
        "{}",
        ui::section_header("Sending note...")
    );
    let id = client
        .create_note(content)
        .await
        .wrap_err("failed to create a note at blob service")?;

    println!(
        "{}",
        ui::success_footer("Note sent successfully")
    );
    println!("Note ID: {}", id);

    Ok(())
}