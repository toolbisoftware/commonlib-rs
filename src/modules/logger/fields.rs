// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[derive(Debug)]
pub struct LogFields {
  pub category: Option<String>,
  pub message: Option<String>,
  pub ms: Option<f64>,
}

impl tracing::field::Visit for LogFields {
  fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
    match field.name() {
      "message" => self.message = Some(format!("{:?}", value)),
      _ => {}
    }
  }

  fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
    match field.name() {
      "category" => self.category = Some(value.to_uppercase().into()),
      "message" => self.message = Some(value.into()),
      _ => {}
    }
  }

  fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
    match field.name() {
      "ms" => self.ms = Some(value),
      _ => {}
    }
  }
}
