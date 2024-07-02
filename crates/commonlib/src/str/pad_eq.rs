// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, hash::Hash};

pub trait PadEq {
  type Output;
  fn pad_eq(&self) -> Self::Output;
}

pub fn pad_eq<T: PadEq>(value: T) -> T::Output {
  value.pad_eq()
}

fn pad_eq_str(string: &str, length: usize) -> String {
  match string.len() {
    str_len if str_len < length => format!("{}{}", string, " ".repeat(length - str_len)),
    _ => string.to_string(),
  }
}

impl<'a> PadEq for Vec<&'a str> {
  type Output = Vec<String>;
  fn pad_eq(&self) -> Self::Output {
    let max_len = self.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut result = Vec::new();
    for value in self {
      result.push(pad_eq_str(value, max_len))
    }

    result
  }
}

impl PadEq for Vec<String> {
  type Output = Vec<String>;
  fn pad_eq(&self) -> Self::Output {
    let max_len = self.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut result = Vec::new();
    for value in self {
      result.push(pad_eq_str(value, max_len))
    }

    result
  }
}

impl<'a, K> PadEq for HashMap<K, &'a str>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn pad_eq(&self) -> Self::Output {
    let max_len = self.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    let mut result = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), pad_eq_str(value, max_len));
    }

    result
  }
}

impl<K> PadEq for HashMap<K, String>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn pad_eq(&self) -> Self::Output {
    let max_len = self.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    let mut result = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), pad_eq_str(value, max_len));
    }

    result
  }
}
