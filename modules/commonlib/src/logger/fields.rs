// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use tracing::field::{Field, Visit};

pub struct Fields {
  pub message: Option<String>,
  pub category: Option<String>,
  pub error: Option<String>,
  pub stopwatch_ms: Option<f64>,
}

impl Visit for Fields {
  fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
    match field.name() {
      "message" => self.message = Some(format!("{:?}", value)),
      "error" => self.error = Some(format!("{:#?}", value)),
      _ => {}
    }
  }

  fn record_str(&mut self, field: &Field, value: &str) {
    match field.name() {
      "message" => self.message = Some(value.to_string()),
      "category" => self.category = Some(value.to_uppercase()),
      _ => {}
    }
  }

  fn record_f64(&mut self, field: &Field, value: f64) {
    match field.name() {
      "ms" => self.stopwatch_ms = Some(value),
      _ => {}
    }
  }
}
