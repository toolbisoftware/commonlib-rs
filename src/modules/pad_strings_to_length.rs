// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::collections::HashMap;

pub fn pad_strings_to_length<K>(strings: HashMap<K, String>, length: usize) -> HashMap<K, String>
where
  K: std::hash::Hash + Clone + Eq,
{
  let mut hashmap: HashMap<K, String> = HashMap::new();

  for (key, value) in strings {
    let value_length: usize = value.len();
    let string: String = match value_length {
      len if len > length => value.chars().take(length).collect(),
      len if len < length => {
        let padding: usize = length - value_length;
        format!("{}{}", value, " ".repeat(padding))
      }
      _ => value,
    };

    hashmap.insert(key, string);
  }

  hashmap
}
