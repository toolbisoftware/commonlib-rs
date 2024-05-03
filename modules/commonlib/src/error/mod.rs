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
pub struct Error {
  pub message: String,
  pub category: Option<String>,
  pub error: Option<Box<dyn StdError + Send>>,
}

#[cfg(feature = "logger")]
pub struct ErrorDisplay<'a>(&'a Error);

impl Error {
  pub fn new(message: &str) -> Self {
    Self {
      message: message.to_string(),
      category: None,
      error: None,
    }
  }

  pub fn category(mut self, category: &str) -> Self {
    self.category = Some(category.to_string());
    self
  }

  pub fn error(mut self, error: Box<dyn StdError + Send>) -> Self {
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

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let message = &self.message;
    let category: String = self
      .category
      .as_ref()
      .map(|category: &String| format!("{}: ", category.to_uppercase()))
      .unwrap_or("".to_string());
    let error: String = self
      .error
      .as_ref()
      .map(|error: &Box<dyn StdError + Send>| format!("\n{}", error.to_string()))
      .unwrap_or("".to_string());

    write!(f, "{}{}{}", category, message, error)?;

    Ok(())
  }
}

impl StdError for Error {}
