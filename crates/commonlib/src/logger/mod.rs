// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{
  collections::HashMap,
  marker::PhantomData,
  path::Path,
  sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use self::{file::FileLog, layer::Layer};

mod fields;
mod file;
mod layer;

#[derive(Debug)]
pub struct Logger<State = Unlocked> {
  level: LogLevel,
  module_filters: HashMap<String, LogLevel>,
  file_logger: FileLogger,
  _state: PhantomData<State>,
}

#[derive(Debug, Clone)]
pub struct FileLogger {
  enable: bool,
  path: String,
  buffer: Arc<Mutex<Vec<FileLog>>>,
}

pub struct Unlocked;

pub struct Locked;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
  #[error("couldn't initialize the logger")]
  Init(tracing_subscriber::util::TryInitError),
  #[error("couldn't initialize the logger: it has a level of 'logger::LogLevel::Off'")]
  InitOff,
}

impl Logger {
  pub fn new() -> Self {
    Self {
      level: LogLevel::Info,
      module_filters: HashMap::new(),
      file_logger: FileLogger {
        enable: false,
        path: "./logs".into(),
        buffer: Arc::new(Mutex::new(Vec::new())),
      },
      _state: PhantomData::<Unlocked>,
    }
  }
}

impl Logger<Unlocked> {
  pub fn set_level(mut self, level: LogLevel) -> Self {
    self.level = level;
    self
  }

  pub fn setup_file_logger(mut self, enable: bool, path: Option<String>) -> Self {
    self.file_logger.enable = enable;

    if let Some(path) = path {
      self.file_logger.path = path;
    }

    self
  }

  pub fn add_module_filter(mut self, module_name: &str, level: LogLevel) -> Self {
    self.module_filters.insert(module_name.into(), level);
    self
  }

  pub fn init(self) -> Result<Logger<Locked>, Error> {
    if self.level == LogLevel::Off {
      return Err(Error::InitOff);
    }

    tracing_subscriber::registry()
      .with(Layer {
        level: self.level,
        module_filters: self.module_filters.clone(),
        file_logger: self.file_logger.clone(),
      })
      .try_init()
      .map_err(Error::Init)?;

    if self.file_logger.enable {
      file::FileLogger::new(
        Path::new(&self.file_logger.path),
        self.file_logger.clone().buffer,
      )
      .init();
    }

    Ok(Logger {
      level: self.level,
      module_filters: self.module_filters,
      file_logger: self.file_logger,
      _state: PhantomData::<Locked>,
    })
  }
}

impl Logger<Locked> {
  pub fn flush() {
    // TODO: Make the 'flush' function
    todo!()
  }

  pub fn stop() {
    // TODO: Make the 'stop' function
    todo!()
  }
}

impl LogLevel {
  fn from_str(string: &str) -> Option<Self> {
    match string {
      "error" => Some(Self::Error),
      "warn" => Some(Self::Warn),
      "info" => Some(Self::Info),
      "debug" => Some(Self::Debug),
      "trace" => Some(Self::Trace),
      _ => None,
    }
  }

  fn to_log_str(self) -> Option<String> {
    match self {
      Self::Error => Some("ERROR".into()),
      Self::Warn => Some("WARN ".into()),
      Self::Info => Some("INFO ".into()),
      Self::Debug => Some("DEBUG".into()),
      Self::Trace => Some("TRACE".into()),
      _ => None,
    }
    .into()
  }
}

impl From<&tracing::Level> for LogLevel {
  fn from(value: &tracing::Level) -> Self {
    match value {
      &tracing::Level::ERROR => Self::Error,
      &tracing::Level::WARN => Self::Warn,
      &tracing::Level::INFO => Self::Info,
      &tracing::Level::DEBUG => Self::Debug,
      &tracing::Level::TRACE => Self::Trace,
    }
  }
}
