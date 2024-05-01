// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use super::Error;
#[cfg(feature = "logger")]
use super::ErrorDisplay;
use tracing::{error, warn};

pub fn soft_panic(error: Error) {
  #[cfg(feature = "logger")]
  let error: ErrorDisplay = error.to_logger();

  error!(error = %error, "An unexpected error has occurred.");
  warn!("Shutting down.");

  std::process::exit(1)
}
