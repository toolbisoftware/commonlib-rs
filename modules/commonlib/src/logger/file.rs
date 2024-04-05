// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Add error handling on the asynchronous logger task
// TODO Add a way to shut down the app safely without losing any logs

use crate::error::{Error, ErrorBuilder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
  fs::File,
  io::{Error as StdError, Read, Write},
  path::{Path, PathBuf},
  sync::Mutex,
  time::Duration,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerFileLog {
  pub timestamp: i64,
  pub level: String,
  pub category: Option<String>,
  pub message: Option<String>,
  pub error: Option<String>,
  pub ms: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LogFile {
  logs: Vec<LoggerFileLog>,
}

//

lazy_static! {
  pub static ref FILE_LOG_BUFFER: Mutex<Vec<LoggerFileLog>> = Mutex::new(Vec::new());
}

//

fn create_dir(path: &Path) -> Result<(), Error> {
  std::fs::create_dir_all(path).map_err(|error: StdError| {
    ErrorBuilder::new("Failed to create the 'logs' directory.")
      .error(&error)
      .get()
  })
}

fn get_file_path(dir_path: &Path, timestamp: &str) -> PathBuf {
  dir_path.join(format!("{}-log.json", timestamp))
}

fn read_file(path: &Path) -> Result<LogFile, Error> {
  let mut file: File = std::fs::OpenOptions::new()
    .create(true)
    .read(true)
    .write(true)
    .open(path)
    .map_err(|error: StdError| {
      ErrorBuilder::new("Failed to read the log file.")
        .error(&error)
        .get()
    })?;
  let mut content: String = String::new();
  let _ = file.read_to_string(&mut content);

  let log_file: LogFile = if content.is_empty() {
    LogFile { logs: Vec::new() }
  } else {
    serde_json::from_str(&content).map_err(|error: serde_json::Error| {
      ErrorBuilder::new("Failed to deserialize the file.")
        .error(&error)
        .get()
    })?
  };

  Ok(log_file)
}

fn write_file(path: &Path, content: &LogFile) -> Result<(), Error> {
  let serialize: String =
    serde_json::to_string_pretty(content).map_err(|error: serde_json::Error| {
      ErrorBuilder::new("Failed to serialize the file.")
        .error(&error)
        .get()
    })?;

  let mut file: File = std::fs::OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(path)
    .map_err(|error: StdError| {
      ErrorBuilder::new("Failed to read the log file.")
        .error(&error)
        .get()
    })?;

  file
    .write_all(serialize.as_bytes())
    .map_err(|error: StdError| {
      ErrorBuilder::new("Failed to write to the log file.")
        .error(&error)
        .get()
    })?;

  Ok(())
}

fn inner_init(path: String) -> Result<(), Error> {
  let exe_path: PathBuf = std::env::current_exe().unwrap();
  let exe_dir_path: &Path = exe_path.parent().unwrap();
  let dir_path_join: PathBuf = exe_dir_path.join(&path);
  let dir_path: &Path = dir_path_join.as_path();

  create_dir(dir_path)?;

  let mut timestamp: String = chrono::Utc::now().format("%Y%m%d").to_string();
  let mut path: PathBuf = get_file_path(dir_path, &timestamp);
  let mut content: LogFile = read_file(path.as_path())?;

  let sleep_dur: Duration = Duration::from_secs(1);

  loop {
    std::thread::sleep(sleep_dur);

    let logs: Vec<LoggerFileLog> = {
      let mut buffer: std::sync::MutexGuard<'_, Vec<LoggerFileLog>> =
        FILE_LOG_BUFFER.lock().unwrap();
      std::mem::take(&mut *buffer)
    };

    for log in logs {
      let log_timestamp_i64: i64 = log.timestamp;
      let log_timestamp_chrono: chrono::prelude::DateTime<chrono::prelude::Utc> =
        chrono::DateTime::from_timestamp_millis(log_timestamp_i64).unwrap();
      let log_timestamp_str: String = log_timestamp_chrono.format("%Y%m%d").to_string();

      if log_timestamp_str != timestamp {
        timestamp = log_timestamp_str;
        path = get_file_path(dir_path, &timestamp);
        content = read_file(path.as_path())?;
      }

      content.logs.push(log);
    }

    write_file(path.as_path(), &content)?;
  }
}

pub fn init(path: &str) -> Result<(), Error> {
  let path: String = path.to_string();
  let mut done: bool = false;

  #[cfg(feature = "logger-async-std")]
  if !done {
    let path: String = path.clone();
    async_std::task::spawn(async move { inner_init(path) });
    done = true;
  }

  #[cfg(feature = "logger-tokio")]
  if !done {
    tokio::task::spawn(async move { inner_init(path) });
    done = true;
  }

  if !done {
    return Err(
      ErrorBuilder::new("Failed to initialize the file logger.")
        .error(&ErrorBuilder::new("Failed to find an asynchronous runtime to use.").get())
        .get(),
    );
  }

  Ok(())
}

pub fn flush(path: &str) -> Result<(), Error> {
  let exe_path: PathBuf = std::env::current_exe().unwrap();
  let exe_dir_path: &Path = exe_path.parent().unwrap();
  let dir_path_join: PathBuf = exe_dir_path.join(&path);
  let dir_path: &Path = dir_path_join.as_path();

  create_dir(dir_path)?;

  let mut timestamp: String = chrono::Utc::now().format("%Y%m%d").to_string();
  let mut path: PathBuf = get_file_path(dir_path, &timestamp);
  let mut content: LogFile = read_file(path.as_path())?;

  let sleep_dur: Duration = Duration::from_secs(1);

  let logs: Vec<LoggerFileLog> = {
    let mut buffer: std::sync::MutexGuard<'_, Vec<LoggerFileLog>> = FILE_LOG_BUFFER.lock().unwrap();
    std::mem::take(&mut *buffer)
  };

  for log in logs {
    let log_timestamp_i64: i64 = log.timestamp;
    let log_timestamp_chrono: chrono::prelude::DateTime<chrono::prelude::Utc> =
      chrono::DateTime::from_timestamp_millis(log_timestamp_i64).unwrap();
    let log_timestamp_str: String = log_timestamp_chrono.format("%Y%m%d").to_string();

    if log_timestamp_str != timestamp {
      timestamp = log_timestamp_str;
      path = get_file_path(dir_path, &timestamp);
      content = read_file(path.as_path())?;
    }

    content.logs.push(log);
  }

  write_file(path.as_path(), &content)?;

  Ok(())
}
