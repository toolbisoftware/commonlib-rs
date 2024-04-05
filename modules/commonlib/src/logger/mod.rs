// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

mod field;
mod file;
mod layer;
mod util;

use self::layer::Layer;
use crate::error::{Error, ErrorBuilder};
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct LoggerInnerFileLogging {
  enabled: bool,
  path: String,
}

#[derive(Clone)]
struct LoggerInner {
  level: tracing::Level,
  file_logging: LoggerInnerFileLogging,
  module_filters: HashMap<String, Level>,
  blocked_modules: Vec<String>,
}

#[derive(Clone)]
pub struct Logger {
  inner: LoggerInner,
}

impl Logger {
  pub fn new() -> Self {
    Self {
      inner: LoggerInner {
        level: Level::INFO,
        file_logging: LoggerInnerFileLogging {
          enabled: false,
          path: "./logs".to_string(),
        },
        module_filters: HashMap::new(),
        blocked_modules: Vec::new(),
      },
    }
  }

  pub fn level(mut self, level: tracing::Level) -> Self {
    self.inner.level = level;
    self
  }

  pub fn file_logging(mut self, value: bool) -> Self {
    self.inner.file_logging.enabled = value;
    self
  }

  pub fn file_logging_path(mut self, value: &str) -> Self {
    self.inner.file_logging.path = value.to_string();
    self
  }

  pub fn add_module_filter(mut self, module: &str, level: Level) -> Self {
    self.inner.module_filters.insert(module.to_string(), level);
    self
  }

  pub fn add_blocked_module(mut self, module: &str) -> Self {
    self.inner.blocked_modules.push(module.to_string());
    self
  }

  pub fn init(&self) -> Result<&Self, Error> {
    if self.inner.file_logging.enabled {
      file::init(&self.inner.file_logging.path)?
    }

    let self_inner: Logger = self.clone();

    tracing_subscriber::registry()
      .with(Layer {
        level: self_inner.inner.level,
        file_logging: self_inner.inner.file_logging,
        module_filters: self_inner.inner.module_filters,
        blocked_modules: self_inner.inner.blocked_modules,
      })
      .try_init()
      .map_err(|_| ErrorBuilder::new("Failed to initialize the logger.").get())?;

    Ok(self)
  }

  pub fn flush(&self) -> Result<(), Error> {
    if self.inner.file_logging.enabled {
      file::flush(&self.inner.file_logging.path)?
    }

    Ok(())
  }
}
