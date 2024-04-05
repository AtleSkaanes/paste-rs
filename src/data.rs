use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::Utc;
use colored::Colorize;

use crate::{api, files};

pub const DATA_FILE_NAME: &str = "DATA";

pub fn get_data_path() -> PathBuf {
    let path: PathBuf;
    if let Some(dir) = directories::ProjectDirs::from("com", "siratle", "paste_rs") {
        path = dir.data_local_dir().to_path_buf()
    } else {
        println!(
            "{}",
            "\u{7}[ERROR]: Can't access Local Data directory!"
                .red()
                .bold()
        );
        std::process::exit(1);
    }

    if !path.exists() {
        if let Err(e) = fs::create_dir_all(path.clone()) {
            println!(
                "{}",
                format!(
                    "\u{7}[ERROR]: Can't create Local Data directory!\nInfo: {}",
                    e.to_string()
                )
                .red()
                .bold()
            );
            std::process::exit(1);
        }
    }

    let path = &path.join(DATA_FILE_NAME);

    if !path.exists() {
        files::write_to_file(path.to_path_buf(), "".to_string(), true);
    }

    path.to_path_buf()
}

pub fn add_data_point(data: &DataPoint) {
    let path = get_data_path();

    files::append_to_file(path, &format!("\n{}\n", data.to_string()));
}

pub fn rewrite_data_points(data: Vec<DataPoint>) {
    let path = get_data_path();

    let out: Vec<String> = data.iter().map(|x| x.to_string()).collect();

    files::write_to_file(path, out.join("\n"), true)
}

pub fn get_local_data() -> Vec<DataPoint> {
    let path = get_data_path();

    let buffer = files::read_file_to_string(path);

    let mut out = vec![];
    for (i, line) in buffer.lines().enumerate() {
        let data_point_result = DataPoint::from_str(line.trim());
        if let Err(e) = &data_point_result {
            println!(
                "{}",
                format!(
                    "\u{7}[ERROR]: While parsing saved data file, at line {}\nInfo: {}",
                    i, e
                )
                .red()
                .bold()
            );
            std::process::exit(1);
        }

        out.push(data_point_result.unwrap());
    }

    out
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub id: String,
    pub url: String,
    pub description: String,
    pub uploadet: chrono::DateTime<Utc>,
}
impl DataPoint {
    pub fn new(id: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            url: api::to_url(id, None),
            description: description.to_string(),
            uploadet: chrono::Utc::now(),
        }
    }
}

impl FromStr for DataPoint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id;
        let description;
        let uploadet;
        match s.split_once(" ") {
            Some((before, after)) => {
                let millis = before.parse::<i64>();
                if millis.is_err() {
                    return Err("Invalid data for timestamp".to_string());
                }
                match chrono::DateTime::<Utc>::from_timestamp_millis(millis.unwrap()) {
                    Some(dt) => uploadet = dt,
                    None => return Err("Invalid millis for timestamp".to_string()),
                }

                match after.split_once(" ") {
                    Some((before, after)) => {
                        id = before;
                        description = after
                            .replace(r"\n", "\n")
                            .replace(r"\r", "\r")
                            .replace(r"\t", "\t")
                            .replace(r"\0", "\0");
                    }
                    None => return Err("Wrong format".to_string()),
                }
            }
            None => return Err("Wrong format".to_string()),
        };

        let data_point = Self {
            id: id.to_string(),
            url: api::to_url(id, None),
            description: description.to_string(),
            uploadet,
        };

        Ok(data_point)
    }
}

impl ToString for DataPoint {
    fn to_string(&self) -> String {
        let description = &self
            .description
            .replace("\n", r"\n")
            .replace("\r", r"\r")
            .replace("\t", r"\t")
            .replace("\0", r"\0");

        format!(
            "{} {} {}",
            self.uploadet.timestamp_millis(),
            self.id,
            description
        )
    }
}
