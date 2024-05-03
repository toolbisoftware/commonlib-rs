// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use super::{
  nester::{create_line, nester},
  ErrorDisplay,
};

impl<'a> std::fmt::Display for ErrorDisplay<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let category: String = self
      .0
      .category
      .as_ref()
      .map(|category: &String| format!("[{}] ", category.to_uppercase()))
      .unwrap_or("".to_string());
    let line: String = create_line(&format!("{}{}", category, self.0.message), 0);
    let line_break = self.0.error.is_some().then(|| "\n").unwrap_or("");

    write!(f, "{}{}", line, line_break)?;

    if let Some(ref error) = self.0.error {
      nester(f, error)?;
    }

    Ok(())
  }
}
