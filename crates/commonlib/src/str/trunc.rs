// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub trait Trunc {
  fn trunc(self, length: usize) -> String;
}

pub fn trunc<T: Trunc>(value: T, length: usize) -> String {
  value.trunc(length)
}

fn trunc_(string: &str, length: usize) -> String {
  match string.len() {
    str_len if str_len > length => string.chars().take(length).collect(),
    _ => string.into(),
  }
}

impl Trunc for &str {
  fn trunc(self, length: usize) -> String {
    trunc_(self, length)
  }
}

impl Trunc for String {
  fn trunc(self, length: usize) -> String {
    trunc_(&self, length)
  }
}
