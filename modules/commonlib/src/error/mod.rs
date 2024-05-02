// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[cfg(feature = "logger")]
mod display;
#[cfg(feature = "logger")]
mod nester;
mod soft_panic;

pub use soft_panic::soft_panic;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error<'a> {
  pub message: &'a str,
  pub category: Option<&'a str>,
  pub error: Option<Box<dyn StdError>>,
}

#[cfg(feature = "logger")]
pub struct ErrorDisplay<'a>(&'a Error<'a>);

impl<'a> Error<'a> {
  pub fn new(message: &'a str) -> Self {
    Self {
      message,
      category: None,
      error: None,
    }
  }

  pub fn category(mut self, category: &'a str) -> Self {
    self.category = Some(category);
    self
  }

  pub fn error(mut self, error: Box<dyn StdError>) -> Self {
    self.error = Some(error);
    self
  }

  pub fn error_str(mut self, error: &'static str) -> Self {
    self.error = Some(Box::new(Error::new(error)));
    self
  }

  #[cfg(feature = "logger")]
  pub fn to_logger(&self) -> ErrorDisplay {
    ErrorDisplay(self)
  }

  #[cfg(feature = "logger")]
  pub fn log(self) -> Self {
    use tracing::error;

    if let Some(ref error) = self.error {
      if let Some(error) = error.downcast_ref::<Error>() {
        let error: ErrorDisplay = error.to_logger();

        error!(
          message = self.message,
          category = self.category,
          error = %error
        );
      } else {
        error!(
          message = self.message,
          category = self.category,
          error = %error
        );
      }
    } else {
      error!(message = self.message, category = self.category);
    }

    self
  }
}

impl<'a> std::fmt::Display for Error<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message: &str = self.message;
    let category: String = self
      .category
      .map(|category: &str| format!("{}: ", category.to_uppercase()))
      .unwrap_or("".to_string());
    let error: String = self
      .error
      .as_ref()
      .map(|error: &Box<dyn StdError>| format!("\n{}", error.to_string()))
      .unwrap_or("".to_string());

    write!(f, "{}{}{}", category, message, error)?;

    Ok(())
  }
}

impl StdError for Error<'static> {}
