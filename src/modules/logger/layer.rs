// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use super::{
  fields::LogFields,
  file::{LoggerLogFileEntry, LOG_BUFFER},
  LoggerFileLoggingBuilder,
};
use crate::modules::pad_string_to_length;
use owo_colors::OwoColorize;
use tracing::Level;

pub struct Layer {
  pub level: Option<Level>,
  pub file_logging: Option<LoggerFileLoggingBuilder>,
}

fn log(level: &Level, log: String) {
  match level {
    &Level::ERROR => {
      eprintln!("{}", log)
    }
    _ => {
      println!("{}", log)
    }
  }
}

impl<S> tracing_subscriber::layer::Layer<S> for Layer
where
  S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
  fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
    let log_level: &Level = event.metadata().level();

    let level: &str = match log_level {
      &Level::ERROR => "ERROR",
      &Level::WARN => "WARN ",
      &Level::INFO => "INFO ",
      &Level::DEBUG => "DEBUG",
      &Level::TRACE => "TRACE",
    };
    let timestamp: chrono::prelude::DateTime<chrono::prelude::Utc> = chrono::Utc::now();
    let timestamp_formatted: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
      timestamp.format("%Y-%m-%d %H:%M:%S");

    let stdout_supports_color: bool = {
      if let Some(support) = supports_color::on(supports_color::Stream::Stdout) {
        if support.has_256 {
          true
        } else {
          false
        }
      } else {
        false
      }
    };
    let stderr_supports_color: bool = {
      if let Some(support) = supports_color::on(supports_color::Stream::Stderr) {
        if support.has_256 {
          true
        } else {
          false
        }
      } else {
        false
      }
    };

    let fields: LogFields = {
      let mut fields: LogFields = LogFields {
        category: None,
        message: None,
        ms: None,
      };
      event.record(&mut fields);

      fields
    };

    let category: &str = {
      if let Some(ref data) = fields.category {
        data
      } else {
        ""
      }
    };
    let message: &str = {
      if let Some(ref data) = fields.message {
        data
      } else {
        ""
      }
    };
    let ms: String = {
      if let Some(data) = fields.ms {
        data.to_string()
      } else {
        "".into()
      }
    };

    if stdout_supports_color && stderr_supports_color {
      let background_color: owo_colors::Style = match log_level {
        &Level::ERROR => owo_colors::Style::new().on_red(),
        &Level::WARN => owo_colors::Style::new().on_yellow(),
        &Level::INFO => owo_colors::Style::new().on_blue(),
        &Level::DEBUG => owo_colors::Style::new().on_magenta(),
        &Level::TRACE => owo_colors::Style::new().on_white(),
      };
      let foreground_color: owo_colors::Style = match log_level {
        &Level::ERROR => owo_colors::Style::new().red(),
        &Level::WARN => owo_colors::Style::new().yellow(),
        &Level::INFO => owo_colors::Style::new().blue(),
        &Level::DEBUG => owo_colors::Style::new().magenta(),
        &Level::TRACE => owo_colors::Style::new().white(),
      };

      let level: String = format!(" | {} ", level);
      let category: String = {
        if category.len() > 0 {
          format!(
            "{}",
            format!("· {} ", pad_string_to_length(&category, 10)).style(background_color)
          )
        } else {
          let category: String = pad_string_to_length(&category, 13);
          format!("{}", category.style(background_color))
        }
      };
      let timestamp: String = format!(" {} ", timestamp_formatted);
      let message: String = {
        if message.len() > 0 {
          format!(" {}", message.style(foreground_color))
        } else {
          message.into()
        }
      };
      let ms: String = {
        if ms.len() > 0 {
          format!(" {}", format!("{} ms", ms).bright_black())
        } else {
          ms.into()
        }
      };

      let text: String = format!(
        "{}{}{}{}{}{}",
        level.style(background_color),
        category,
        timestamp.on_bright_black(),
        " ".style(background_color),
        message.style(foreground_color),
        ms.bright_black()
      );

      log(log_level, text);
    } else {
      let category: String = {
        if category.len() > 0 {
          format!("{}", format!("· {} ", pad_string_to_length(&category, 10)))
        } else {
          pad_string_to_length(&category, 13)
        }
      };
      let ms: String = {
        if ms.len() > 0 {
          format!(" {}", format!("{} ms", ms))
        } else {
          ms
        }
      };

      let text: String = format!(
        "{} {} · {} · {}{}",
        level, category, timestamp_formatted, message, ms
      );

      log(log_level, text);
    }

    if let Some(file_logging) = self.file_logging.to_owned() {
      if !file_logging.enabled {
        return;
      }

      let log: LoggerLogFileEntry = LoggerLogFileEntry {
        timestamp: timestamp.timestamp_millis(),
        level: log_level.to_string(),
        category: fields.category,
        message: fields.message,
        ms: fields.ms,
      };

      LOG_BUFFER.lock().unwrap().push(log);
    }

    ctx.event(event);
  }
}
