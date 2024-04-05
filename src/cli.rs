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
    Send {
        content: String,
        #[arg(short = 'f', long = "file")]
        is_file: bool,
    },
    Get {
        id: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        extenstion: Option<String>,
    },
    Delete {
        id: String,
    },
    Open {
        id: String,
        #[arg(short, long)]
        extenstion: Option<String>,
    },
}
