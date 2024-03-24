// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use super::{
  field::Fields,
  file::{LoggerFileLog, FILE_LOG_BUFFER},
  util::{get_env_var_level, log_to_console},
  LoggerInnerFileLogging,
};
use crate::str::pad_len;
use owo_colors::{OwoColorize, Style};
use std::{collections::HashMap, env};
use tracing::Level;

pub struct Layer {
  pub level: Level,
  pub file_logging: LoggerInnerFileLogging,
  pub module_filters: HashMap<String, Level>,
  pub blocked_modules: Vec<String>,
}

impl<S> tracing_subscriber::layer::Layer<S> for Layer
where
  S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
  fn enabled(
    &self,
    metadata: &tracing::Metadata<'_>,
    _ctx: tracing_subscriber::layer::Context<'_, S>,
  ) -> bool {
    // Check if the module is blocked
    if let Some(module_path) = metadata.module_path() {
      if self
        .blocked_modules
        .iter()
        .position(|x| x == module_path)
        .is_some()
      {
        return false;
      };
    }

    // Get the log level
    let log_level: &Level = metadata.level();

    let module_filter: Option<&Level> = metadata
      .module_path()
      .map(|v| self.module_filters.get(v))
      .unwrap_or(None);

    // Get the global level
    let env_var_level: String = env::var("LOG_LEVEL").unwrap_or("".to_string());
    let env_level: Option<Level> = get_env_var_level(env_var_level);
    let level: Level = env_level.unwrap_or(self.level);

    if let Some(module_filter) = module_filter {
      // Inverted for some reason
      let valid: bool = log_level <= module_filter;

      if !valid {
        return false;
      }

      if &level < log_level {
        let env_var_force: String = env::var("LOG_LEVEL_FORCE").unwrap_or("".to_string());
        if env_var_force != "0" {
          return false;
        }
      }

      return true;
    }

    if &level < log_level {
      return false;
    };

    true
  }

  fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
    let level: &Level = event.metadata().level();
    let level_str: &str = match level {
      &Level::ERROR => "ERROR",
      &Level::WARN => "WARN ",
      &Level::INFO => "INFO ",
      &Level::DEBUG => "DEBUG",
      &Level::TRACE => "TRACE",
    };

    let timestamp: chrono::prelude::DateTime<chrono::prelude::Utc> = chrono::Utc::now();
    let timestamp_str: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
      timestamp.format("%Y-%m-%d %H:%M:%S");

    let stdout_color_support: bool = supports_color::on(supports_color::Stream::Stdout)
      .map(|s| s.has_256)
      .unwrap_or(false);
    let stderr_color_support: bool = supports_color::on(supports_color::Stream::Stderr)
      .map(|s| s.has_256)
      .unwrap_or(false);

    let fields: Fields = {
      let mut result: Fields = Fields {
        message: None,
        category: None,
        error: None,
        ms: None,
      };

      event.record(&mut result);

      result
    };

    let message: String = fields.message.clone().unwrap_or("".to_string());
    let category: String = fields.category.clone().unwrap_or("".to_string());
    let error: String = fields
      .error
      .clone()
      .map(|v| format!("\n{}", v))
      .unwrap_or("".to_string());
    let ms: String = fields.ms.map(|v| v.to_string()).unwrap_or("".to_string());

    //

    if stdout_color_support && stderr_color_support {
      let bg_color: Style = match level {
        &Level::ERROR => Style::new().on_red(),
        &Level::WARN => Style::new().on_yellow(),
        &Level::INFO => Style::new().on_blue(),
        &Level::DEBUG => Style::new().on_magenta(),
        &Level::TRACE => Style::new().on_white(),
      };
      let fg_color: Style = match level {
        &Level::ERROR => Style::new().red(),
        &Level::WARN => Style::new().yellow(),
        &Level::INFO => Style::new().blue(),
        &Level::DEBUG => Style::new().magenta(),
        &Level::TRACE => Style::new().white(),
      };

      let level_str: String = format!(" | {} ", level_str);
      let category: String = if !category.is_empty() {
        format!(
          "{}",
          format!("· {} ", pad_len(&*category, 10)).style(bg_color)
        )
      } else {
        let pad: String = pad_len(&*category, 13);
        format!("{}", pad.style(bg_color))
      };
      let timestamp: String = format!(" {} ", timestamp_str);
      let message: String = if !message.is_empty() {
        format!(" {}", message.style(fg_color))
      } else {
        message.to_string()
      };
      let ms: String = if !ms.is_empty() {
        format!(" {}", format!("{} ms", ms).bright_black())
      } else {
        ms.to_string()
      };

      let log: String = format!(
        "{}{}{}{}{}{}{}",
        level_str.style(bg_color),
        category,
        timestamp.on_bright_black(),
        " ".style(bg_color),
        message.style(fg_color), // ! ?
        ms.bright_black(),
        error
      );

      log_to_console(level, log);
    } else {
      let category: String = if !category.is_empty() {
        format!("{}", format!("· {} ", pad_len(&*category, 10)))
      } else {
        pad_len(&*category, 13)
      };
      let ms: String = if !ms.is_empty() {
        format!(" {}", format!("{} ms", ms))
      } else {
        ms
      };

      let log: String = format!(
        "{} {} · {} · {}{}{}",
        level_str, category, timestamp_str, message, ms, error
      );

      log_to_console(level, log);
    }

    if self.file_logging.enabled {
      FILE_LOG_BUFFER.lock().unwrap().push(LoggerFileLog {
        timestamp: timestamp.timestamp_millis(),
        level: level.to_string(),
        category: fields.category,
        message: fields.message,
        error: fields.error,
        ms: fields.ms,
      })
    }

    ctx.event(event);
  }
}
