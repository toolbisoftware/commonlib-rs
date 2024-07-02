// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub trait Trunc {
  fn trunc(self, length: usize) -> String;
}

pub fn trunc(string: &str, length: usize) -> String {
  match string.len() {
    str_len if str_len > length => string.chars().take(length).collect(),
    _ => string.to_string(),
  }
}

impl Trunc for &str {
  fn trunc(self, length: usize) -> String {
    trunc(self, length)
  }
}

impl Trunc for String {
  fn trunc(self, length: usize) -> String {
    trunc(&self, length)
  }
}
