use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    SendFile {
        file_path: PathBuf,
    },
    Send {
        content: String,
    },
    Get {
        id: u64,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Open {
        id: u64,
    },
}
