use std::{
    io::{BufRead, IsTerminal},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Send {
        content: Option<String>,
        #[arg(short = 'f', long = "file")]
        is_file: bool,
    },
    Get {
        id: Option<String>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        extenstion: Option<String>,
    },
    Delete {
        id: Option<String>,
    },
    Open {
        id: Option<String>,
        #[arg(short, long)]
        extenstion: Option<String>,
    },
}

pub fn get_stdin(input_name: &str) -> String {
    if std::io::stdin().is_terminal() {
        println!(
            "{}",
            format!("\u{7}[ERROR]: Missing input \"{}\"", input_name)
                .red()
                .bold()
        );
        std::process::exit(1);
    }
    let input = std::io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

    input
}
