use std::path::PathBuf;

use chrono::Utc;
use clap::Parser;
use colored::Colorize;

use crate::{cli::CliArgs, data::DataPoint};

pub mod api;
mod cli;
pub mod data;
pub mod files;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.subcommand {
        cli::SubCommand::Send {
            content,
            is_file,
            description,
        } => {
            let content = match content {
                Some(c) => c,
                None => cli::get_stdin("content"),
            };
            let data = match is_file {
                true => {
                    let path = PathBuf::from(content);
                    files::read_text_from_file(path)
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

            let description = if description.is_some() {
                description.unwrap()
            } else {
                "[NO DESCRIPTION]".to_owned()
            };

            let data_point = DataPoint::new(&response.id, &description);

            data::add_data_point(&data_point);

            if response.status_code == 206 {
                println!(
                    "{}",
                    "The data exceeded paste.rs' limit, so some if it has been cut off!"
                        .yellow()
                        .bold()
                        .italic()
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
            extension,
        } => {
            let id = match id {
                Some(id) => id,
                None => cli::get_stdin("id"),
            };

            let id = match extension {
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
                let path = files::save_text_to_file(path.clone(), response.clone().text, &id);
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

            let data_points: Vec<DataPoint> = data::get_local_data()
                .iter()
                .filter_map(|x| match x.id != id {
                    true => Some(x.clone()),
                    false => None,
                })
                .collect();

            match api::send_delete_request(id.clone()).await {
                Ok(_) => {
                    println!("Succesfully deleted data with id \"{}\"", id);
                    data::rewrite_data_points(data_points);
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
        cli::SubCommand::Open { id, extension } => {
            let id = match id {
                Some(id) => id,
                None => cli::get_stdin("id"),
            };

            let url = api::to_url(&id, extension.as_deref());
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
        cli::SubCommand::List => {
            let data_points = data::get_local_data();

            if data_points.len() == 0 {
                println!("You have currently no saved data!");
                std::process::exit(0);
            }

            println!("Showing {} data points\n", data_points.len());

            println!(
                "{:<20} | {:<10} | {:<25} | {}",
                "Uploaded".blue().bold(),
                "Id".blue().bold(),
                "Url".blue().bold(),
                "Description".blue().bold()
            );

            let today = Utc::now().format("%Y/%m/%d").to_string();
            for point in data_points.iter().rev() {
                let temp_formatted = point.uploadet.format("%Y/%m/%d").to_string();
                let formatted_time = if today == temp_formatted {
                    point.uploadet.format("%H:%M:%S").to_string()
                } else {
                    temp_formatted
                };
                println!(
                    "{:<20} | {:<10} | {:<25} | {}",
                    formatted_time, point.id, point.url, point.description
                )
            }
        }
    };
}
