// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::collections::HashMap;

pub fn pad_strings_to_longest<K>(strings: HashMap<K, String>) -> HashMap<K, String>
where
  K: std::hash::Hash + Clone + Eq,
{
  let mut max_length = 0;
  let mut hashmap: HashMap<K, String> = HashMap::new();

  for (_key, value) in strings.iter() {
    let value_length: usize = value.len();
    if value_length > max_length {
      max_length = value_length;
    }
  }

  for (key, value) in strings.iter() {
    let value_length: usize = value.len();
    let string = match value_length {
      len if len < max_length => {
        let padding = max_length - value_length;
        format!("{}{}", value, " ".repeat(padding))
      }
      _ => value.to_owned(),
    };

    hashmap.insert(key.clone(), string);
  }

  hashmap
}
