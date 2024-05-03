// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Maybe return a different struct after init

mod fields;
mod file_logger;
mod layer;
use self::{
  file_logger::{FileLogger, Log},
  layer::Layer,
};
use std::{
  collections::HashMap,
  path::Path,
  result::Result as StdResult,
  sync::{Arc, Mutex},
};
use thiserror::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug)]
pub struct Logger {
  log_level: LogLevel,
  file_logger: LoggerFileLogger,
  module_filters: HashMap<String, LogLevel>,
  log_buffer: Arc<Mutex<Vec<Log>>>,
}

#[derive(Debug, Clone)]
struct LoggerFileLogger {
  enable: bool,
  path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
  Off,
}

#[derive(Debug, Error)]
pub enum Error {
  #[error("Failed to initialize the logger.")]
  Init(tracing_subscriber::util::TryInitError),
  #[error("Failed to initialize the file logger.")]
  FileLoggerInit(self::file_logger::Error),
}

type Result<T> = StdResult<T, Error>;

impl Logger {
  pub fn new() -> Self {
    Self {
      log_level: LogLevel::Info,
      file_logger: LoggerFileLogger {
        enable: false,
        path: "./logs".to_string(),
      },
      module_filters: HashMap::new(),
      log_buffer: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub fn init(self) -> Result<Self> {
    tracing_subscriber::registry()
      .with(Layer {
        log_level: self.log_level.clone(),
        file_logger: self.file_logger.clone(),
        module_filters: self.module_filters.clone(),
        log_buffer: self.log_buffer.clone(),
      })
      .try_init()
      .map_err(Error::Init)?;

    Ok(self)
  }

  pub async fn init_file_logger(&self) -> Result<()> {
    FileLogger::new(Path::new(&self.file_logger.path), self.log_buffer.clone())
      .init()
      .await
      .map_err(Error::FileLoggerInit)?;

    Ok(())
  }

  pub fn stop(&self) -> Result<()> {
    todo!()
  }

  pub fn flush(&self) -> Result<()> {
    todo!()
  }

  pub fn set_level(mut self, level: LogLevel) -> Self {
    self.log_level = level;
    self
  }

  pub fn enable_file_logger(mut self, enable: bool) -> Self {
    self.file_logger.enable = enable;
    self
  }

  pub fn set_file_logger_path(mut self, file_logger_path: &str) -> Self {
    self.file_logger.path = file_logger_path.to_string();
    self
  }

  pub fn add_module_filter(mut self, module_name: &str, log_level: LogLevel) -> Self {
    self
      .module_filters
      .insert(module_name.to_string(), log_level);
    self
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

  pub fn from_tracing_level(tracing_level: &tracing::Level) -> Self {
    match tracing_level {
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
