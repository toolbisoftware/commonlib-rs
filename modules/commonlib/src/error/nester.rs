// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use super::Error;
use owo_colors::OwoColorize;
use std::error::Error as StdError;

pub fn nester(
  f: &mut std::fmt::Formatter<'_>,
  error: &Box<dyn StdError + Send>,
) -> std::fmt::Result {
  nester_inner(f, error, 1)
}

fn nester_inner(
  f: &mut std::fmt::Formatter<'_>,
  error: &Box<dyn StdError + Send>,
  deepness: usize,
) -> std::fmt::Result {
  match error.downcast_ref::<Error>() {
    Some(error) => {
      let category: String = error
        .category
        .as_ref()
        .map(|category: &String| format!("[{}] ", category.to_uppercase()))
        .unwrap_or("".to_string());
      let line: String = create_line(&format!("{}{}", category, error.message), deepness);
      let line_break: &str = error.error.is_some().then(|| "\n").unwrap_or("");

      write!(f, "{}{}", line, line_break)?;

      if let Some(ref error) = error.error {
        nester_inner(f, &error, deepness + 1)?;
      }
    }
    None => {
      let source: Option<&dyn StdError> = error.source();
      let line: String = create_line(&error.to_string(), deepness);
      let line_break: &str = source.is_some().then(|| "\n").unwrap_or("");

      write!(f, "{}{}", line, line_break)?;

      if let Some(ref error) = source {
        nester_inner_std(f, error, deepness + 1)?;
      }
    }
  };

  Ok(())
}

fn nester_inner_std(
  f: &mut std::fmt::Formatter<'_>,
  error: &dyn StdError,
  deepness: usize,
) -> std::fmt::Result {
  let source: Option<&dyn StdError> = error.source();
  let line: String = create_line(&error.to_string(), deepness);
  let line_break: &str = source.is_some().then(|| "\n").unwrap_or("");

  write!(f, "{}{}", line, line_break)?;

  if let Some(source) = source {
    nester_inner_std(f, source, deepness + 1)?;
  }

  Ok(())
}

pub fn create_line(string: &str, deepness: usize) -> String {
  match deepness {
    deepness if deepness == 0 => {
      format!(" {} {}", "·".bright_black(), string.red())
    }
    _ => {
      format!(
        "{}{} {}",
        " ".repeat(deepness),
        "|·".bright_black(),
        string.red()
      )
    }
  }
}
