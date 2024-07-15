// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO: Handle flushing and stopping the logger

use std::{
  fs::{self, File, OpenOptions},
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
  time::Duration,
};

use csv::{Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::LogLevel;

#[derive(Debug)]
pub struct FileLogger {
  path: PathBuf,
  buffer: Arc<Mutex<Vec<FileLog>>>,
  // writer: Option<Arc<Mutex<Writer<File>>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileLog {
  pub timestamp: i64,
  pub level: LogLevel,
  pub category: Option<String>,
  pub message: Option<String>,
  pub stopwatch: Option<String>,
  pub error: Option<String>,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("couldn't create the 'logs' directory")]
  CreateDir(std::io::Error),
  #[error("couldn't open or create the log file")]
  OpenFile(std::io::Error),
  #[error("couldn't serialize the log")]
  Serialize(csv::Error),
}

impl FileLogger {
  pub fn new(path: &Path, buffer: Arc<Mutex<Vec<FileLog>>>) -> Self {
    let mut dir_path = std::env::current_exe()
      .unwrap()
      .parent()
      .unwrap()
      .to_path_buf();
    dir_path.push(path);
    dir_path = fs::canonicalize(dir_path).unwrap();

    Self {
      path: dir_path,
      buffer,
      // writer: None,
    }
  }

  // TODO: Do this better
  pub fn init(self) {
    async_std::task::spawn(async move { Self::init_(self).unwrap() });
  }

  pub async fn init_async(self) -> Result<(), Error> {
    async_std::task::spawn(async move { Self::init_(self) }).await
  }

  fn init_(self) -> Result<(), Error> {
    Self::create_dir(&self.path)?;

    let mut timestamp = chrono::Utc::now().format("%Y%m%d").to_string();
    let mut path = Self::get_file_path(&self.path, &timestamp);
    let file = Arc::new(Mutex::new(Self::create_writer(&path)?));

    let sleep_dur = Duration::from_secs(1);

    loop {
      std::thread::sleep(sleep_dur);

      let logs = {
        let mut buffer = self.buffer.lock().unwrap();

        std::mem::take(&mut *buffer)
      };

      for log in logs {
        let log_timestamp = {
          let date_time = chrono::DateTime::from_timestamp_millis(log.timestamp).unwrap();

          date_time.format("%Y%m%d").to_string()
        };

        if log_timestamp != timestamp {
          file.lock().unwrap().flush().unwrap(); // TODO: Handle this posible error

          timestamp = log_timestamp;
          path = Self::get_file_path(&self.path, &timestamp);

          let get_file = Self::create_writer(&path)?;
          let mut file_mutex = file.lock().unwrap();
          *file_mutex = get_file;
        }

        file
          .lock()
          .unwrap()
          .serialize(log)
          .map_err(Error::Serialize)?;
      }

      file.lock().unwrap().flush().unwrap(); // TODO: Handle this posible error
    }
  }

  fn create_dir(path: &Path) -> Result<(), Error> {
    fs::create_dir_all(path).map_err(Error::CreateDir)
  }

  fn create_writer(path: &Path) -> Result<Writer<File>, Error> {
    let file_exists = path.metadata().is_ok();
    let file = OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .open(path)
      .map_err(Error::OpenFile)?;

    Ok(
      WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file),
    )
  }

  fn get_file_path(dir_path: &Path, timestamp: &str) -> PathBuf {
    dir_path.join(format!("{}-log.csv", timestamp))
  }
}
