// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, hash::Hash};

fn run(string: &str, length: usize) -> String {
  let str_len: usize = string.len();
  match str_len {
    str_len if str_len < length => format!("{}{}", string, " ".repeat(length - str_len)),
    _ => string.to_string(),
  }
}

pub trait PadEq {
  type Output;
  fn run(&self) -> Self::Output;
}

impl<'a, K> PadEq for HashMap<K, &'a str>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn run(&self) -> Self::Output {
    let length: usize = self.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    let mut result: HashMap<K, String> = HashMap::new();
    for (key, string) in self {
      result.insert(key.to_owned(), run(string, length));
    }

    result
  }
}

impl<'a> PadEq for Vec<&'a str> {
  type Output = Vec<String>;
  fn run(&self) -> Self::Output {
    let length: usize = self.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut result: Vec<String> = Vec::new();
    for string in self {
      result.push(run(string, length));
    }

    result
  }
}

pub fn pad_eq<T: PadEq>(value: T) -> T::Output {
  value.run()
}
