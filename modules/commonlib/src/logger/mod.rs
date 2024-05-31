// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{
  collections::HashMap,
  path::Path,
  result::Result as StdResult,
  sync::{Arc, Mutex},
};

use thiserror::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use self::file::FileLog;

mod fields;
mod file;
mod layer;

type Result<T = ()> = StdResult<T, Error>;

#[derive(Debug)]
pub struct Logger {
  level: LogLevel,
  file_logger_enable: bool,
  file_logger_path: String,
  file_logger_buffer: Arc<Mutex<Vec<FileLog>>>,
  module_filters: HashMap<String, LogLevel>,
}

pub struct LoggerBuilder {
  inner: Logger,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
  Off,
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("Failed to initialize the logger.")]
  Init(tracing_subscriber::util::TryInitError),
  #[error("Failed to initialize the file logger.")]
  FileLoggerInit(self::file::Error),
}

impl Logger {
  pub fn new() -> LoggerBuilder {
    LoggerBuilder::new()
  }

  pub fn init(self) -> Result<Self> {
    tracing_subscriber::registry()
      .with(Layer {
        level: self.level.clone(),
        file_logger_enable: self.file_logger_enable,
        file_logger_path: self.file_logger_path.clone(),
        file_logger_buffer: self.file_logger_buffer.clone(),
        module_filters: self.module_filters.clone(),
      })
      .try_init()
      .map_err(Error::Init)?;

    Ok(self)
  }

  pub async fn init_file_logger(&self) -> Result {
    FileLogger::new(
      Path::new(&self.file_logger_path),
      self.file_logger_buffer.clone(),
    )
    .init()
    .await
    .map_err(Error::FileLoggerInit)?;

    Ok(())
  }
}

impl LoggerBuilder {
  pub fn new() -> Self {
    Self {
      inner: Logger {
        level: LogLevel::Info,
        file_logger_enable: false,
        file_logger_path: "./logs".to_string(),
        file_logger_buffer: Arc::new(Mutex::new(Vec::new())),
        module_filters: HashMap::new(),
      },
    }
  }

  pub fn level(mut self, level: LogLevel) -> Self {
    self.inner.level = level;
    self
  }

  pub fn file_logger(mut self, enable: bool) -> Self {
    self.inner.file_logger_enable = enable;
    self
  }

  pub fn file_logger_path(mut self, path: &str) -> Self {
    self.inner.file_logger_path = path.to_string();
    self
  }

  pub fn module_filter(mut self, name: &str, level: LogLevel) -> Self {
    self.inner.module_filters.insert(name.to_string(), level);
    self
  }

  pub fn get(self) -> Logger {
    self.inner
  }
}

impl LogLevel {
  pub fn from_str(string: &str) -> Option<Self> {
    match string {
      "error" => Some(Self::Error),
      "warn" => Some(Self::Warn),
      "info" => Some(Self::Info),
      "debug" => Some(Self::Debug),
      "trace" => Some(Self::Trace),
      _ => None,
    }
  }

  pub fn from_tracing_level(level: &tracing::Level) -> Self {
    match level {
      &tracing::Level::ERROR => Self::Error,
      &tracing::Level::WARN => Self::Warn,
      &tracing::Level::INFO => Self::Info,
      &tracing::Level::DEBUG => Self::Debug,
      &tracing::Level::TRACE => Self::Trace,
    }
  }

  pub fn to_log_str<'a>(self) -> Option<&'a str> {
    match self {
      Self::Error => Some("ERROR"),
      Self::Warn => Some("WARN "),
      Self::Info => Some("INFO "),
      Self::Debug => Some("DEBUG"),
      Self::Trace => Some("TRACE"),
      _ => None,
    }
  }

  pub fn to_str<'a>(self) -> Option<&'a str> {
    match self {
      Self::Error => Some("error"),
      Self::Warn => Some("warn"),
      Self::Info => Some("info"),
      Self::Debug => Some("debug"),
      Self::Trace => Some("trace"),
      _ => None,
    }
  }
}
