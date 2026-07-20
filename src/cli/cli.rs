use clap::{Parser,Subcommand};
use crate::cli::init::init_project;
use crate::cli::run::run_project;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {},
    Run {
        target: Option<String>
    }
}

pub async fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Init {} => {
            init_project().await?
        },
        Commands::Run { target } => {
            run_project(target).await?
        }
    }

    Ok(())
}