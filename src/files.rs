use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use colored::Colorize;

use crate::api;

pub fn read_text_from_file(path: PathBuf) -> String {
    if !path.exists() {
        println!(
            "{}",
            "\u{7}[ERROR]: The given path doesn't exist".red().bold()
        );
        std::process::exit(1);
    }
    if path.is_dir() {
        println!(
            "{}",
            "\u{7}[ERROR]: The given path is to a directory. Can only read from files!"
                .red()
                .bold()
        );
        std::process::exit(1);
    }

    read_file_to_string(path)
}

pub fn save_text_to_file(path: PathBuf, text: String, id: &str) -> PathBuf {
    let mut path = path;

    if let Err(e) = fs::create_dir_all(path.clone().parent().unwrap()) {
        println!(
            "{}",
            format!("\u{7}[ERROR]: Coulnd't create the output files' parent directories, at {:?}\nInfo: {}",
                    path,
                    e.to_string()
            )
            .red()
            .bold()
        );
        std::process::exit(1);
    };

    if path.exists() && path.is_dir() {
        path = path.join(format!("{}.txt", api::strip_id(id)));
    }

    if path.exists() {
        for i in 0..512 {
            let file_name = path
                .clone()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or("")
                .to_string();

            path.set_file_name(format!("{}({})", file_name, i));
            if !path.exists() {
                break;
            }
        }
    }

    write_to_file(path.clone(), text, true);

    path
}

pub fn write_to_file(path: PathBuf, content: String, can_create: bool) {
    let mut file = match OpenOptions::new()
        .create(can_create)
        .write(true)
        .truncate(true)
        .open(path.clone())
    {
        Ok(f) => f,
        Err(e) => {
            println!(
                "{}",
                format!(
                    "\u{7}[ERROR]: Failed to open file at {:?}!\nInfo: {}",
                    path,
                    e.to_string()
                )
                .red()
                .bold(),
            );
            std::process::exit(1);
        }
    };

    if let Err(e) = file.write_all(content.as_bytes()) {
        println!(
            "{}",
            format!(
                "\u{7}[ERROR]: Failed to write to the output file at {:?}!\nInfo: {}",
                path,
                e.to_string()
            )
            .red()
            .bold()
        )
    }
}

pub fn append_to_file(path: PathBuf, content: &str) {
    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(path.clone())
    {
        Ok(f) => f,
        Err(e) => {
            println!(
                "{}",
                format!(
                    "\u{7}[ERROR]: Failed to open file at {:?}!\nInfo: {}",
                    path,
                    e.to_string()
                )
                .red()
                .bold(),
            );
            std::process::exit(1);
        }
    };

    if let Err(e) = file.write_all(content.as_bytes()) {
        println!(
            "{}",
            format!(
                "\u{7}[ERROR]: Failed to write to the output file at {:?}!\nInfo: {}",
                path,
                e.to_string()
            )
            .red()
            .bold()
        )
    }
}

pub fn read_file_to_string(path: PathBuf) -> String {
    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("{}", "\u{7}[ERROR]: Coulnd't open the file".red().bold());
            std::process::exit(1);
        }
    };

    let mut buf = String::new();
    let read_result = file.read_to_string(&mut buf);

    if read_result.is_err() {
        println!(
            "{}",
            "\u{7}[ERROR]: Couldn't read from the file".red().bold()
        );
        std::process::exit(1);
    }

    buf
}
