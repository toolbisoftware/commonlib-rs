// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[derive(Debug)]
pub struct LogFields {
  pub message: Option<String>,
  pub category: Option<String>,
  pub stopwatch: Option<String>,
  pub error: Option<String>,
}

impl LogFields {
  pub fn new() -> Self {
    Self {
      message: None,
      category: None,
      stopwatch: None,
      error: None,
    }
  }
}

impl tracing::field::Visit for LogFields {
  fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
    match field.name() {
      "message" | "msg" => self.message = Some(format!("{:?}", value)),
      "error" | "err" => self.error = Some(format!("{:?}", value)),
      _ => {}
    }
  }

  fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
    match field.name() {
      "message" | "msg" => self.message = Some(value.into()),
      "category" | "cat" => self.category = Some(value.into()),
      "stopwatch" | "sw" => self.stopwatch = Some(value.into()),
      _ => {}
    }
  }
}
