// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
  fs::File,
  io::{Read, Write},
  path::{Path, PathBuf},
  result::Result as StdResult,
  sync::{Arc, Mutex, MutexGuard},
  time::Duration,
};
use thiserror::Error as ThisError;

pub struct FileLogger {
  dir_path: PathBuf,
  log_buffer: Arc<Mutex<Vec<Log>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
  pub timestamp: i64,
  pub level: String,
  pub category: Option<String>,
  pub message: Option<String>,
  pub error: Option<String>,
  pub ms: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogFile {
  logs: Vec<Log>,
}

#[derive(Debug, ThisError)]
pub enum Error {
  #[error("Failed to create the 'logs' directory.")]
  CreateDir(std::io::Error),
  #[error("Failed to read the log file.")]
  ReadFile(std::io::Error),
  #[error("Failed to deserialize the log file.")]
  DeserializeFile(serde_json::Error),
  #[error("Failed to serialize the log file.")]
  SerializeFile(serde_json::Error),
  #[error("Failed to write to the log file.")]
  WriteFile(std::io::Error),
}

type Result<T> = StdResult<T, Error>;

impl FileLogger {
  pub fn new(path: &Path, log_buffer: Arc<Mutex<Vec<Log>>>) -> Self {
    let exe_path: PathBuf = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .join(path);

    Self {
      dir_path: exe_path,
      log_buffer,
    }
  }

  pub async fn init(self) -> Result<()> {
    Self::create_logs_dir(&self.dir_path)?;

    let mut current_date_timestamp: String = Utc::now().format("%Y%m%d").to_string();
    let mut file_path: PathBuf = Self::get_log_file_path(&self.dir_path, &current_date_timestamp);
    let mut file_content: LogFile = Self::read_log_file(&file_path)?;

    let sleep_dur: Duration = Duration::from_secs(1);

    loop {
      std::thread::sleep(sleep_dur);

      let logs: Vec<Log> = {
        let mut buffer: MutexGuard<Vec<Log>> = self.log_buffer.lock().unwrap();
        std::mem::take(&mut *buffer)
      };

      for log in logs {
        let timestamp: String = {
          let timestamp_i64: i64 = log.timestamp;
          let timestamp_chrono: DateTime<Utc> =
            chrono::DateTime::from_timestamp_millis(timestamp_i64).unwrap();
          timestamp_chrono.format("%Y%m%d").to_string()
        };

        if timestamp != current_date_timestamp {
          Self::write_log_file(&file_path, &file_content)?;

          current_date_timestamp = timestamp;
          file_path = Self::get_log_file_path(&self.dir_path, &current_date_timestamp);
          file_content = Self::read_log_file(&file_path)?;
        }

        file_content.logs.push(log);
      }

      Self::write_log_file(&file_path, &file_content)?;
    }
  }

  fn create_logs_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path).map_err(Error::CreateDir)?;
    Ok(())
  }

  fn get_log_file_path(dir_path: &Path, timestamp: &str) -> PathBuf {
    dir_path.join(format!("{}-log.json", timestamp))
  }

  fn read_log_file(path: &Path) -> Result<LogFile> {
    let content: String = {
      let mut file: File = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .map_err(Error::ReadFile)?;

      let mut result: String = String::new();
      file.read_to_string(&mut result).unwrap();
      result
    };

    if content.is_empty() {
      Ok(LogFile { logs: Vec::new() })
    } else {
      serde_json::from_str(&content).map_err(Error::DeserializeFile)
    }
  }

  fn write_log_file(path: &Path, content: &LogFile) -> Result<()> {
    let mut file: File = std::fs::OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(path)
      .map_err(Error::ReadFile)?;
    let serialize: String = serde_json::to_string_pretty(&content).map_err(Error::SerializeFile)?;

    file
      .write_all(serialize.as_bytes())
      .map_err(Error::WriteFile)?;

    Ok(())
  }
}
