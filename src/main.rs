use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

use crate::cli::CliArgs;

pub mod api;
mod cli;
pub mod files;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.subcommand {
        cli::SubCommand::Send { content, is_file } => {
            let content = match content {
                Some(c) => c,
                None => cli::get_stdin("content"),
            };
            let data = match is_file {
                true => {
                    let path = PathBuf::from(content);
                    files::read_from_file(path)
                }
                false => content,
            };

            let response = match api::send_post_request(data).await {
                Ok(resp) => resp,
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "\u{7}[ERROR]: Error while sending data to paste.rs!\nInfo: {}",
                            e.to_string()
                        )
                        .red()
                        .bold()
                    );
                    std::process::exit(1);
                }
            };

            if response.status_code == 206 {
                println!(
                    "{}",
                    "The data exceeded paste.rs' limit, so some if it has been cut off!"
                        .yellow()
                        .bold()
                );
            }

            println!(
                "Data saved to\nID:\t{}\nURL:\t{}",
                response.id, response.url
            );
        }
        cli::SubCommand::Get {
            id,
            output,
            extenstion,
        } => {
            let id = match id {
                Some(id) => id,
                None => cli::get_stdin("id"),
            };

            let id = match extenstion {
                Some(ext) => format!("{}.{}", id, ext),
                None => id,
            };
            let response = match api::send_get_request(api::strip_id(&id).to_string()).await {
                Ok(resp) => resp,
                Err(e) => {
                    println!("{}", format!("\u{7}[ERROR]: Can't get the data from the specified ID, did you type it correctly?\nInfo: {}", e.to_string()).red().bold());
                    std::process::exit(1);
                }
            };

            if let Some(path) = output {
                let path = files::write_to_file(path.clone(), response.clone().text, &id);
                println!("Data saved to file {:?}", path.file_name().unwrap());
                std::process::exit(0);
            }

            println!("{}", response.text);
            std::process::exit(0);
        }
        cli::SubCommand::Delete { id } => {
            let id = match id {
                Some(id) => id,
                None => cli::get_stdin("id"),
            };

            let id = api::strip_id(&id).to_string();
            match api::send_delete_request(id.clone()).await {
                Ok(_) => {
                    println!("Succesfully deleted data with id \"{}\"", id)
                }
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "\u{7}[ERROR]: Failed to complete Delete request!\nInfo: {}",
                            e.to_string()
                        )
                        .red()
                        .bold()
                    )
                }
            }
        }
        cli::SubCommand::Open { id, extenstion } => {
            let id = match id {
                Some(id) => id,
                None => cli::get_stdin("id"),
            };

            let url = api::to_url(&id, extenstion.as_deref());
            let open_result = open::that(url.clone());

            if let Err(_) = open_result {
                println!(
                    "{}{}",
                    "\u{7}[ERROR]: Couldn't open the requested ID with the full URL: "
                        .red()
                        .bold(),
                    url.red().bold()
                );
            }
        }
    };
}
