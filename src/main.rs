mod internal;
mod cli;

use clap::Parser;
use crate::cli::cli::{dispatch, Cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    dispatch(cli).await?;

    Ok(())
}