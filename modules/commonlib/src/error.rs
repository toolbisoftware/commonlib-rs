// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
  pub message: String,
  pub category: Option<String>,
  pub error: Option<String>,
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message = &self.message;
    let error = match &self.error {
      Some(error) => format!("\n{}", error),
      None => "".to_string(),
    };

    write!(f, "{}{}", message, error)
  }
}

impl std::error::Error for Error {}

//

pub struct ErrorBuilder {
  inner: Error,
}

impl ErrorBuilder {
  pub fn new(message: &str) -> Self {
    Self {
      inner: Error {
        message: message.to_string(),
        category: None,
        error: None,
      },
    }
  }

  pub fn category(mut self, category: &str) -> Self {
    self.inner.category = Some(category.to_string());
    self
  }

  pub fn error(mut self, error: &dyn std::error::Error) -> Self {
    self.inner.error = Some(error.to_string());
    self
  }

  pub fn get(self) -> Error {
    self.inner
  }
}
