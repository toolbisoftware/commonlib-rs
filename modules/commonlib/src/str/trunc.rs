// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub fn trunc(string: String, max_length: usize) -> String {
  let str_len: usize = string.len();
  match str_len {
    str_len if str_len > max_length => string.chars().take(max_length).collect(),
    _ => string.to_string(),
  }
}
