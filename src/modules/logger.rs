// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Implement 'tracing' instead of 'env_logger' for more performance.
// TODO Log to files that have the date and the log level in their name.
// TODO Add in-code documentation.

use owo_colors::OwoColorize;

pub struct Logger {}

impl Logger {
  fn log(level: log::Level, log: String) {
    match level {
      log::Level::Error => {
        eprintln!("{}", log);
      }
      _ => {
        println!("{}", log);
      }
    }
  }

  pub fn init(level: Option<&str>) -> Result<(), std::io::Error> {
    let manual_level: &str = level.unwrap_or("info");
    let env: env_logger::Env<'_> = env_logger::Env::default().filter_or("LOG_LEVEL", manual_level);

    env_logger::Builder::from_env(env)
      .format(|_, record: &log::Record<'_>| {
        Ok({
          let log_level: log::Level = record.level();
          let timestamp: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
          let level: &str = match log_level {
            log::Level::Error => "ERROR",
            log::Level::Warn => "WARN ",
            log::Level::Info => "INFO ",
            log::Level::Debug => "DEBUG",
            log::Level::Trace => "TRACE",
          };

          let mut stdout_supports_color: bool = false;
          let mut stderr_supports_color: bool = false;

          if let Some(support) = supports_color::on(supports_color::Stream::Stdout) {
            if support.has_16m || support.has_256 {
              stdout_supports_color = true;
            }
          }
          if let Some(support) = supports_color::on(supports_color::Stream::Stderr) {
            if support.has_16m || support.has_256 {
              stderr_supports_color = true;
            }
          }

          if stdout_supports_color && stderr_supports_color {
            let background_color: owo_colors::Style = match log_level {
              log::Level::Error => owo_colors::Style::new().on_red(),
              log::Level::Warn => owo_colors::Style::new().on_yellow(),
              log::Level::Info => owo_colors::Style::new().on_blue(),
              log::Level::Debug => owo_colors::Style::new().on_magenta(),
              log::Level::Trace => owo_colors::Style::new().on_white(),
            };
            let foreground_color: owo_colors::Style = match log_level {
              log::Level::Error => owo_colors::Style::new().red(),
              log::Level::Warn => owo_colors::Style::new().yellow(),
              log::Level::Info => owo_colors::Style::new().blue(),
              log::Level::Debug => owo_colors::Style::new().magenta(),
              log::Level::Trace => owo_colors::Style::new().white(),
            };

            let log_text_1: String = format!(" | {} ", level);
            let log_text_2: String = format!(" {} ", timestamp);

            let formatted_log: String = format!(
              "{}{}{} {}",
              log_text_1.style(background_color),
              log_text_2.on_bright_black(),
              " ".style(background_color),
              record.args().style(foreground_color)
            );

            Logger::log(log_level, formatted_log);
          } else {
            let formatted_log: String = format!("{} · {} · {}", level, timestamp, record.args());

            Logger::log(log_level, formatted_log);
          }
        })
      })
      .init();

    Ok(())
  }
}
