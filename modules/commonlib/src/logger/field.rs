// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub struct Fields {
  pub message: Option<String>,
  pub category: Option<String>,
  pub error: Option<String>,
  pub ms: Option<f64>,
}

impl tracing::field::Visit for Fields {
  fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
    match field.name() {
      "message" => self.message = Some(format!("{:?}", value)),
      "error" => self.error = Some(format!("{:#?}", value)),
      _ => {}
    }
  }

  fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
    match field.name() {
      "message" => self.message = Some(value.to_string()),
      "category" => self.category = Some(value.to_uppercase().to_string()),
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
