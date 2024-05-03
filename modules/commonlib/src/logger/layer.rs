// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Maybe add a different log level for file logs

use super::{fields::Fields, file_logger::Log, LogLevel, LoggerFileLogger};
use crate::str::pad_len;
use chrono::{
  format::{DelayedFormat, StrftimeItems},
  DateTime, Utc,
};
use owo_colors::{OwoColorize, Style};
use std::{
  collections::HashMap,
  env,
  sync::{Arc, Mutex},
};
use tracing::{Event, Metadata, Subscriber};
use tracing_subscriber::{
  layer::{Context, Layer as TracingLayer},
  registry::LookupSpan,
};

pub struct Layer {
  pub log_level: LogLevel,
  pub file_logger: LoggerFileLogger,
  pub module_filters: HashMap<String, LogLevel>,
  pub log_buffer: Arc<Mutex<Vec<Log>>>,
}

impl<S> TracingLayer<S> for Layer
where
  S: Subscriber + for<'span> LookupSpan<'span>,
{
  fn enabled(&self, metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
    if self.log_level == LogLevel::Off {
      return false;
    }

    let logger_log_level: LogLevel = {
      let env_level_str: String = env::var("LOG_LEVEL").unwrap_or("".to_string());
      LogLevel::from_str(&env_level_str).unwrap_or(self.log_level)
    };
    let log_level: LogLevel = LogLevel::from_tracing_level(metadata.level());

    let module_log_level: Option<&LogLevel> = metadata
      .module_path()
      .map(|module_path: &str| self.module_filters.get(module_path))
      .unwrap_or(None);

    match module_log_level {
      Some(module_log_level) => match &log_level <= module_log_level {
        true => {
          if logger_log_level < log_level {
            let env_enforce_log_level: String =
              env::var("LOG_LEVEL_FORCE").unwrap_or("".to_string());
            if env_enforce_log_level != "0" {
              return false;
            }
          }

          true
        }
        false => false,
      },
      None => logger_log_level >= log_level,
    }
  }

  fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
    let log_level: LogLevel = LogLevel::from_tracing_level(event.metadata().level());
    let timestamp: DateTime<Utc> = chrono::Utc::now();
    let fields: Fields = {
      let mut fields: Fields = Fields {
        message: None,
        category: None,
        error: None,
        stopwatch_ms: None,
      };
      event.record(&mut fields);
      fields
    };

    let log_str_level: &str = log_level.to_log_str().unwrap();
    let log_str_category: String = fields.category.clone().unwrap_or("".to_string());
    let log_str_timestamp: DelayedFormat<StrftimeItems> = timestamp.format("%Y-%m-%d %H:%M:%S");
    let log_str_message: String = fields.message.clone().unwrap_or("".to_string());
    let log_str_error: String = fields
      .error
      .clone()
      .map(|error: String| format!("\n{}", error))
      .unwrap_or("".to_string());
    let log_str_stopwatch_ms = fields
      .stopwatch_ms
      .map(|ms: f64| ms.to_string())
      .unwrap_or("".to_string());

    let terminal_color_support: bool = {
      supports_color::on(supports_color::Stream::Stdout)
        .map(|v: supports_color::ColorLevel| v.has_256)
        .unwrap_or(false)
        && supports_color::on(supports_color::Stream::Stderr)
          .map(|v: supports_color::ColorLevel| v.has_256)
          .unwrap_or(false)
    };

    let log_str = match terminal_color_support {
      true => {
        let bg_color: Style = match log_level {
          LogLevel::Error => Some(Style::new().on_red()),
          LogLevel::Warn => Some(Style::new().on_yellow()),
          LogLevel::Info => Some(Style::new().on_blue()),
          LogLevel::Debug => Some(Style::new().on_magenta()),
          LogLevel::Trace => Some(Style::new().on_white()),
          _ => None,
        }
        .unwrap();

        let fg_color: Style = match log_level {
          LogLevel::Error => Some(Style::new().red()),
          LogLevel::Warn => Some(Style::new().yellow()),
          LogLevel::Info => Some(Style::new().blue()),
          LogLevel::Debug => Some(Style::new().magenta()),
          LogLevel::Trace => Some(Style::new().white()),
          _ => None,
        }
        .unwrap();

        let log_str_level: String = format!(" | {} ", log_str_level);
        let log_str_category: String = log_str_category
          .is_empty()
          .then(|| format!("{}", pad_len(log_str_category.clone(), 13).style(bg_color)))
          .unwrap_or(format!(
            "{}",
            format!("· {} ", pad_len(log_str_category, 10)).style(bg_color),
          ));
        let log_str_timestamp: String = format!(" {} ", log_str_timestamp);
        let log_str_message: String = log_str_message
          .is_empty()
          .then(|| log_str_message.to_string())
          .unwrap_or(format!(" {}", log_str_message.style(fg_color)));
        let log_str_stopwatch_ms: String = log_str_stopwatch_ms
          .is_empty()
          .then(|| log_str_stopwatch_ms.to_string())
          .unwrap_or(format!(
            " {}",
            format!("{} ms", log_str_stopwatch_ms).bright_black()
          ));

        format!(
          "{}{}{}{}{}{}{}",
          log_str_level.style(bg_color),
          log_str_category,
          log_str_timestamp.on_bright_black(),
          " ".style(bg_color),
          log_str_message.style(fg_color),
          log_str_stopwatch_ms.bright_black(),
          log_str_error
        )
      }
      false => {
        let log_str_category: String = log_str_category
          .is_empty()
          .then(|| pad_len(log_str_category.clone(), 13))
          .unwrap_or(format!("· {} ", pad_len(log_str_category, 10)));
        let log_str_stopwatch_ms: String = log_str_message
          .is_empty()
          .then(|| log_str_stopwatch_ms.to_string())
          .unwrap_or(format!(" {} ms", log_str_stopwatch_ms));

        format!(
          "{} {} · {} · {}{}{}",
          log_str_level,
          log_str_category,
          log_str_timestamp,
          log_str_message,
          log_str_stopwatch_ms,
          log_str_error
        )
      }
    };

    match log_level {
      LogLevel::Error => {
        eprintln!("{}", log_str)
      }
      _ => println!("{}", log_str),
    }

    if self.file_logger.enable {
      self.log_buffer.lock().unwrap().push(Log {
        timestamp: timestamp.timestamp_millis(),
        level: log_level.to_str().unwrap().to_string(),
        category: fields.category,
        message: fields.message,
        error: fields.error,
        ms: fields.stopwatch_ms,
      })
    }

    ctx.event(event)
  }
}
