// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::io::Error;

#[derive(Debug)]
pub struct CommonError<'a> {
  pub message: &'a str,
  pub error: Option<Error>,
}

impl std::fmt::Display for CommonError<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message: String = format!("{}", self.message);
    let error: String = if let Some(value) = &self.error {
      format!("\n{}", value)
    } else {
      "".into()
    };

    write!(f, "{}{}", message, error)
  }
}
