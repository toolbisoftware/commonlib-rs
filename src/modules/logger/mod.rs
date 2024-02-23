// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

mod fields;
mod file;
mod layer;

use self::layer::Layer;
use crate::error::CommonError;
use std::io::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct LoggerFileLoggingBuilder {
  pub enabled: bool,
  pub path: Option<&'static str>,
}

pub struct LoggerBuilder {
  pub level: Option<tracing::Level>,
  pub file_logging: Option<LoggerFileLoggingBuilder>,
}

pub struct Logger {
  pub level: Option<tracing::Level>,
  pub file_logging: Option<LoggerFileLoggingBuilder>,
}

impl Logger {
  pub fn init(builder: LoggerBuilder) -> Result<(), Error> {
    let env_filter: tracing_subscriber::EnvFilter = tracing_subscriber::EnvFilter::builder()
      .with_default_directive(builder.level.unwrap_or(tracing::Level::INFO).into())
      .with_env_var("LOG_LEVEL")
      .from_env_lossy();

    if let Some(file_logging) = &builder.file_logging {
      if file_logging.enabled {
        let path: &str = if let Some(path) = file_logging.path {
          path
        } else {
          "./logs"
        };

        self::file::init(path)
          .map_err(|error: Error| CommonError {
            message: "The file logger couldn't be initialized.",
            error: Some(error),
          })
          .unwrap();
      }
    }

    tracing_subscriber::registry()
      .with(env_filter)
      .with(Layer {
        level: builder.level,
        file_logging: builder.file_logging,
      })
      .try_init()
      .ok();

    Ok(())
  }
}
