// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, hash::Hash};

pub trait PadLen {
  type Output;
  fn pad_len(&self, length: usize) -> Self::Output;
}

pub fn pad_len<T: PadLen>(value: T, length: usize) -> T::Output {
  value.pad_len(length)
}

fn pad_len_str(string: &str, length: usize) -> String {
  match string.len() {
    str_len if str_len > length => string.chars().take(length).collect(),
    str_len if str_len < length => format!("{}{}", string, " ".repeat(length - str_len)),
    _ => string.to_string(),
  }
}

impl PadLen for &str {
  type Output = String;
  fn pad_len(&self, length: usize) -> Self::Output {
    pad_len_str(self, length)
  }
}

impl PadLen for String {
  type Output = String;
  fn pad_len(&self, length: usize) -> Self::Output {
    pad_len_str(self, length)
  }
}

impl<'a> PadLen for Vec<&'a str> {
  type Output = Vec<String>;
  fn pad_len(&self, length: usize) -> Self::Output {
    let mut result = Vec::new();
    for value in self {
      result.push(pad_len_str(value, length))
    }
    result
  }
}

impl PadLen for Vec<String> {
  type Output = Vec<String>;
  fn pad_len(&self, length: usize) -> Self::Output {
    let mut result = Vec::new();
    for value in self {
      result.push(pad_len_str(value, length))
    }
    result
  }
}

impl<'a, K> PadLen for HashMap<K, &'a str>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn pad_len(&self, length: usize) -> Self::Output {
    let mut result = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), pad_len_str(value, length));
    }
    result
  }
}

impl<K> PadLen for HashMap<K, String>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn pad_len(&self, length: usize) -> Self::Output {
    let mut result = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), pad_len_str(value, length));
    }
    result
  }
}
