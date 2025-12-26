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
pub struct GetNoteArgs {
    pub id: i32,
}

pub async fn run(
    GetNoteArgs {
        id,
    }: GetNoteArgs,
    mut ctx: Context,
) -> eyre::Result<()> {
    println!(
        "{}",
        ui::header("🎯 FETCHING NOTE 🎯")
    );

    let url = ctx.config()?.blob_url;
    println!("Id: {}\nBlob-url: {}", id, url);

    println!(
        "{}",
        ui::section_header("Connecting...")
    );
    let mut client = BlobClient::new(url);
    client.connect().await?;

    println!(
        "{}",
        ui::section_header("Fetching note...")
    );
    let content = client
        .get_note(id)
        .await
        .wrap_err("failed to fetch a note from the blob service")?;

    println!(
        "{}",
        ui::success_footer("Note fetched successfully")
    );
    println!("Note: {}", content);

    Ok(())
}