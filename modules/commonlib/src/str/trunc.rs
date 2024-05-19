// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub trait Trunc {
  fn trunc(self, max_length: usize) -> String;
}

pub fn trunc(string: &str, max_length: usize) -> String {
  let str_len: usize = string.len();
  match str_len {
    str_len if str_len > max_length => string.chars().take(max_length).collect(),
    _ => string.to_string(),
  }
}

impl Trunc for &str {
  fn trunc(self, max_length: usize) -> String {
    trunc(self, max_length)
  }
}

impl Trunc for String {
  fn trunc(self, max_length: usize) -> String {
    trunc(&self, max_length)
  }
}
