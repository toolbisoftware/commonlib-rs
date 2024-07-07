// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Add 'anyhow' compatibility

use std::{error::Error as _, ops::Deref as _, panic::Location};

pub use self::fmt::Group;

mod fmt;

#[derive(Debug)]
pub struct Error {
  message: String,
  category: Option<String>,
  trace: Option<ErrorTrace>,
  kind: ErrorKind,
}

#[derive(Debug)]
pub struct ErrorTrace {
  file: String,
  line: String,
  column: String,
}

#[derive(Debug)]
pub enum ErrorKind {
  Original,
  Error(Box<dyn std::error::Error>),
  Source(Box<dyn std::error::Error>),
}

impl Error {
  #[track_caller]
  pub fn new(message: &str) -> Self {
    let caller = std::panic::Location::caller();

    Self {
      message: message.to_string(),
      category: None,
      trace: Some(ErrorTrace::from(caller)),
      kind: ErrorKind::Original,
    }
  }

  // TODO Move this into the `From` trait when Rust's issue #50133 is fixed
  pub fn from_error<T: std::error::Error + 'static>(error: T) -> Self {
    Self {
      message: error.to_string(),
      category: None,
      trace: None,
      kind: ErrorKind::Error(Box::new(error)),
    }
  }

  pub fn category(mut self, category: &str) -> Self {
    self.category = Some(category.to_string());
    self
  }

  // TODO Disallow the usage of this function when it has a value
  pub fn cause<T: std::error::Error + 'static>(mut self, error: T) -> Self {
    self.kind = ErrorKind::Source(Box::new(error));
    self
  }

  // TODO Add 'log' function

  pub fn panic(self) -> ! {
    panic!("{}", self)
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match &self.kind {
      ErrorKind::Original => None,
      ErrorKind::Error(error) => error.source(),
      ErrorKind::Source(source) => Some(source.deref()),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut errors = Vec::new();
    errors.push((self.message.clone(), &self.category, &self.trace));

    let mut current_error = self.source();
    while let Some(error) = current_error {
      match error.downcast_ref::<Error>() {
        Some(error) => errors.push((error.message.clone(), &error.category, &error.trace)),
        None => {
          let error_str = error.to_string().clone();
          errors.push((error_str, &None, &None))
        }
      };

      current_error = error.source();
    }

    let mut groups = Vec::new();
    for error in errors {
      let mut elements = Vec::new();
      let category = error
        .1
        .as_ref()
        .map(|c| format!(" ({})", c))
        .unwrap_or("".to_string());

      elements.push(format!("error{}: {}", category, error.0));

      if let Some(trace) = error.2 {
        elements.push(format!("at {}:{}:{}", trace.file, trace.line, trace.column));
      }

      groups.push(Group { elements })
    }

    let lines = fmt::format(groups);
    for line in lines {
      writeln!(f, "{}", line)?;
    }

    Ok(())
  }
}

impl ErrorTrace {
  pub fn new() -> Self {
    Self {
      file: file!().to_string(),
      line: line!().to_string(),
      column: column!().to_string(),
    }
  }
}

impl From<&Location<'_>> for ErrorTrace {
  fn from(value: &Location) -> Self {
    Self {
      file: value.file().to_string(),
      line: value.line().to_string(),
      column: value.column().to_string(),
    }
  }
}
