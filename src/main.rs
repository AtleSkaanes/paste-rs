use clap::Parser;

use crate::cli::CliArgs;

mod cli;

fn main() {
    let args = CliArgs::parse();

    match args.subcommand {
        cli::SubCommand::SendFile { file_path } => unimplemented!(),
        cli::SubCommand::Send { content } => unimplemented!(),
        cli::SubCommand::Get { id, output } => unimplemented!(),
        cli::SubCommand::Open { id } => unimplemented!(),
    }

    println!("Hello, world!");
}
