// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use tracing::Level;

pub fn get_env_var_level(value: String) -> Option<Level> {
  match value.to_lowercase().as_str() {
    "error" => Some(Level::ERROR),
    "warn" => Some(Level::WARN),
    "info" => Some(Level::INFO),
    "debug" => Some(Level::DEBUG),
    "trace" => Some(Level::TRACE),
    _ => None,
  }
}

pub fn log_to_console(level: &Level, log: String) {
  match level {
    &Level::ERROR => {
      eprintln!("{}", log)
    }
    _ => {
      println!("{}", log)
    }
  }
}
