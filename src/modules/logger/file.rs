// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Maybe make all the logs to go to another task so the logger it's even faster.

use crate::error::CommonError;
use serde::{Deserialize, Serialize};
use std::{
  io::{Error, Read, Write},
  path::{Path, PathBuf},
  sync::Mutex,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerLogFileEntry {
  pub timestamp: String,
  pub level: String,
  pub category: Option<String>,
  pub message: Option<String>,
  pub ms: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogFile {
  logs: Vec<LoggerLogFileEntry>,
}

lazy_static::lazy_static! {
  pub static ref LOG_BUFFER: Mutex<Vec<LoggerLogFileEntry>> = Mutex::new(Vec::<LoggerLogFileEntry>::new());
}

fn create_dir(path: &Path) -> Result<(), Error> {
  std::fs::create_dir_all(path)
}

fn get_file_path(dir_path: &Path, timestamp: &str) -> PathBuf {
  dir_path.join(format!("{}-log.json", timestamp))
}

fn read_file(path: &Path) -> Result<LogFile, Error> {
  let mut file: std::fs::File = std::fs::OpenOptions::new()
    .create(true)
    .read(true)
    .write(true)
    .open(path)?;
  let mut content: String = String::new();

  let _ = file.read_to_string(&mut content);

  if content.is_empty() {
    Ok(LogFile { logs: Vec::new() })
  } else {
    let deserialize: LogFile = serde_json::from_str(&content)?;
    Ok(deserialize)
  }
}

fn write_file(path: &Path, content: &LogFile) -> Result<(), Error> {
  let serialize: String = serde_json::to_string_pretty(&content)?;
  let serialize: &[u8] = serialize.as_bytes();

  let mut file = std::fs::OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(path)?;

  file.write_all(serialize)?;

  Ok(())
}

pub fn init(base_path: &'static str) -> Result<(), Error> {
  async_std::task::spawn(async move {
    let dir_path: &Path = Path::new(base_path);

    create_dir(&dir_path)
      .map_err(|error: Error| CommonError {
        message: "Couldn't create the 'logs' directory.",
        error: Some(error),
      })
      .unwrap();

    let mut timestamp: String = chrono::Utc::now().format("%Y%m%d").to_string();
    let mut file_path: PathBuf = get_file_path(dir_path, &timestamp);
    let mut file_content: LogFile = read_file(file_path.as_path())
      .map_err(|error: Error| CommonError {
        message: "Couldn't read the log file.",
        error: Some(error),
      })
      .unwrap();

    let sleep_duration: std::time::Duration = std::time::Duration::from_secs(1);
    loop {
      std::thread::sleep(sleep_duration);

      let logs: Vec<LoggerLogFileEntry> = {
        let mut log_buffer: std::sync::MutexGuard<'_, Vec<LoggerLogFileEntry>> =
          LOG_BUFFER.lock().unwrap();
        std::mem::take(&mut *log_buffer)
      };

      for log in logs {
        let log_timestamp: &String = &log.timestamp;
        let log_timestamp: chrono::prelude::NaiveDateTime =
          chrono::NaiveDateTime::parse_from_str(log_timestamp, "%Y-%m-%d %H:%M:%S").unwrap();
        let log_timestamp: String = log_timestamp.format("%Y%m%d").to_string();

        if &log_timestamp != &timestamp {
          timestamp = log_timestamp.to_owned();
          file_path = get_file_path(dir_path, &timestamp);
          file_content = read_file(file_path.as_path())
            .map_err(|error: Error| CommonError {
              message: "Couldn't read the log file.",
              error: Some(error),
            })
            .unwrap();
        }

        file_content.logs.push(log);
      }

      write_file(&file_path, &file_content)
        .map_err(|error: Error| CommonError {
          message: "Couldn't write to the log file.",
          error: Some(error),
        })
        .unwrap();
    }
  });

  Ok(())
}
