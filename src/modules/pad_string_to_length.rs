// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub fn pad_string_to_length(string: String, length: usize) -> String {
  let string_length: usize = string.len();
  let string: String = match string_length {
    len if len > length => string.chars().take(length).collect(),
    len if len < length => {
      let padding: usize = length - string_length;
      format!("{}{}", string, " ".repeat(padding))
    }
    _ => string,
  };

  string
}
