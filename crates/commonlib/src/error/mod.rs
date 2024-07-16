// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO: Add 'anyhow' compatibility
// TODO: Add color to the errors whenever the 'colorful-logs' feature is enabled

use std::ops::Deref;

pub use commonlib_proc_macros::error;

mod display;

#[derive(Debug)]
pub struct Error {
  message: String,
  category: Option<String>,
  location: Option<Location>,
  source: Option<SourceKind>,
}

#[derive(Debug)]
pub struct Location {
  file: String,
  line: String,
  column: String,
}

#[derive(Debug)]
pub enum SourceKind {
  Error(Box<dyn std::error::Error>),
  FromError(Box<dyn std::error::Error>),
}

impl Error {
  #[track_caller]
  pub fn new(message: &str) -> Self {
    let caller = std::panic::Location::caller();

    Self {
      message: message.into(),
      category: None,
      location: Some(Location::from(caller)),
      source: None,
    }
  }

  // TODO: Move this to a 'From' trait when the issue https://github.com/rust-lang/rust/issues/50133 is solved
  pub fn from_error<T: std::error::Error + 'static>(error: T) -> Self {
    Self {
      message: error.to_string(),
      category: None,
      location: None,
      source: Some(SourceKind::FromError(Box::new(error))),
    }
  }

  pub fn set_category(mut self, category: &str) -> Self {
    self.category = Some(category.into());
    self
  }

  /// ## WARNING!
  /// Using this method will overwrite the previous value.
  pub fn set_source<T: std::error::Error + 'static>(mut self, error: T) -> Self {
    self.source = Some(SourceKind::Error(Box::new(error)));
    self
  }

  // Add the 'log' function

  pub fn panic(self) -> ! {
    panic!("{}", self)
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match &self.source {
      Some(source) => {
        match source {
          SourceKind::Error(error) => Some(error.deref()),
          // ! Might not work
          SourceKind::FromError(error) => error.source(),
        }
      }
      &None => None,
    }
  }
}

impl Location {
  fn new(file: String, line: String, column: String) -> Self {
    Self { file, line, column }
  }
}

impl From<&std::panic::Location<'_>> for Location {
  fn from(value: &std::panic::Location<'_>) -> Self {
    Self::new(
      value.file().into(),
      value.line().to_string(),
      value.column().to_string(),
    )
  }
}
