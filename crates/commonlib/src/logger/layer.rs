// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, env};

use tracing::Subscriber;
use tracing_subscriber::registry::LookupSpan;

use crate::str::PadLen;

use super::{fields::LogFields, FileLog, FileLogger, LogLevel};

#[derive(Debug)]
pub struct Layer {
  pub level: LogLevel,
  pub module_filters: HashMap<String, LogLevel>,
  pub file_logger: FileLogger,
}

fn log_to_console(level: LogLevel, string: &str) {
  match level {
    LogLevel::Error => {
      eprintln!("{}", string)
    }
    _ => println!("{}", string),
  }
}

impl<S> tracing_subscriber::layer::Layer<S> for Layer
where
  S: Subscriber + for<'span> LookupSpan<'span>,
{
  fn enabled(
    &self,
    metadata: &tracing::Metadata<'_>,
    _ctx: tracing_subscriber::layer::Context<'_, S>,
  ) -> bool {
    let logger_level = {
      let env_level_str = env::var("LOG_LEVEL").unwrap_or("".into());

      LogLevel::from_str(&env_level_str).unwrap_or(self.level)
    };

    let module_level = metadata
      .module_path()
      .map(|m| self.module_filters.get(m))
      .unwrap_or(None);

    let log_level = LogLevel::from(metadata.level());

    match module_level {
      Some(module_level) => match module_level >= &log_level {
        true => {
          if logger_level < log_level {
            // Whether to enforce the global level or not
            // TODO: Add option in the builder
            let enforce_level = env::var("LOG_LEVEL_FORCE").unwrap_or("".into());

            // If enforce level is not '0' don't log it
            if enforce_level != "0" {
              return false;
            }
          }

          true
        }
        false => false,
      },
      None => logger_level >= log_level,
    }
  }

  fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
    let log_level = LogLevel::from(event.metadata().level());
    let timestamp = chrono::Utc::now();
    let fields = {
      let mut result = LogFields::new();
      event.record(&mut result);
      result
    };

    let str_level = log_level.to_log_str().unwrap();
    let str_category = fields.category.clone().unwrap_or("".into());
    let str_timestamp = timestamp.format("%Y-%m-%d %H:%M:%S");
    let str_message = fields.message.clone().unwrap_or("".into());
    let str_stopwatch = fields.stopwatch.clone().unwrap_or("".into());
    let str_error = fields
      .error
      .as_ref()
      .map(|e| format!("\n{}", e))
      .unwrap_or("".into());

    let mut console_logged = false;

    #[cfg(feature = "colorful-logs")]
    {
      use owo_colors::{OwoColorize, Style};

      let terminal_color_support = supports_color::on_cached(supports_color::Stream::Stdout)
        .map(|a| a.has_256)
        .unwrap_or(false)
        && supports_color::on_cached(supports_color::Stream::Stderr)
          .map(|a| a.has_256)
          .unwrap_or(false);

      if terminal_color_support {
        let bg_color = match log_level {
          LogLevel::Error => Some(Style::new().on_red()),
          LogLevel::Warn => Some(Style::new().on_yellow()),
          LogLevel::Info => Some(Style::new().on_blue()),
          LogLevel::Debug => Some(Style::new().on_magenta()),
          LogLevel::Trace => Some(Style::new().on_white()),
          _ => None,
        }
        .unwrap();

        let fg_color = match log_level {
          LogLevel::Error => Some(Style::new().red()),
          LogLevel::Warn => Some(Style::new().yellow()),
          LogLevel::Info => Some(Style::new().default_color()),
          LogLevel::Debug => Some(Style::new().magenta()),
          LogLevel::Trace => Some(Style::new().white()),
          _ => None,
        }
        .unwrap();

        let str_level = format!(" | {} ", str_level);
        let str_category = str_category
          .is_empty()
          .then(|| str_category.pad_len(13).style(bg_color).to_string())
          .unwrap_or(
            format!("· {} ", str_category.pad_len(10))
              .style(bg_color)
              .to_string(),
          );

        let str_timestamp = format!(" {} ", str_timestamp);
        let str_message = str_message
          .is_empty()
          .then(|| str_message.to_string())
          .unwrap_or(format!(" {}", str_message.style(fg_color)));

        let str_stopwatch = str_stopwatch
          .is_empty()
          .then(|| str_stopwatch.to_string())
          .unwrap_or(format!(" {}", str_stopwatch.bright_black()));

        let console_log = format!(
          "{}{}{}{}{}{}{}",
          str_level.style(bg_color),
          str_category,
          str_timestamp.on_bright_black(),
          " ".style(bg_color),
          str_message,
          str_stopwatch,
          str_error
        );

        log_to_console(log_level, &console_log);

        console_logged = true;
      }
    }

    if !console_logged {
      let str_level = format!("{} ", str_level);
      let str_category = str_category
        .is_empty()
        .then(|| str_category.pad_len(13))
        .unwrap_or(format!("· {} ", str_category.pad_len(10)).to_string());

      let str_timestamp = format!("· {}", str_timestamp);
      let str_message = str_message
        .is_empty()
        .then(|| str_message.to_string())
        .unwrap_or(format!("· {}", str_message));

      let str_stopwatch = str_stopwatch
        .is_empty()
        .then(|| str_stopwatch.to_string())
        .unwrap_or(format!(" {}", str_stopwatch));

      let console_log = format!(
        "{}{}{}{}{}{}{}",
        str_level, str_category, str_timestamp, " ", str_message, str_stopwatch, str_error
      );

      log_to_console(log_level, &console_log);
    }

    if self.file_logger.enable {
      let file_log = FileLog {
        timestamp: timestamp.timestamp_millis(),
        level: log_level,
        category: fields.category,
        message: fields.message,
        stopwatch: fields.stopwatch,
        error: fields.error,
      };

      let mut buffer = self.file_logger.buffer.lock().unwrap();
      buffer.push(file_log)
    }

    // ! Why?
    ctx.event(event)
  }
}
