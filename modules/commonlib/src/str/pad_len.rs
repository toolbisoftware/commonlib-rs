// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, hash::Hash};

pub trait PadLen {
  type Output;
  fn run(&self, length: usize) -> Self::Output;
}

pub fn pad_len<T: PadLen>(value: T, length: usize) -> T::Output {
  value.run(length)
}

impl PadLen for &str {
  type Output = String;
  fn run(&self, length: usize) -> Self::Output {
    string(self, length)
  }
}

impl PadLen for String {
  type Output = String;
  fn run(&self, length: usize) -> Self::Output {
    string(self, length)
  }
}

impl<'a> PadLen for Vec<&'a str> {
  type Output = Vec<String>;
  fn run(&self, length: usize) -> Self::Output {
    let mut result: Vec<String> = Vec::new();
    for value in self {
      result.push(string(value, length))
    }
    result
  }
}

impl PadLen for Vec<String> {
  type Output = Vec<String>;
  fn run(&self, length: usize) -> Self::Output {
    let mut result: Vec<String> = Vec::new();
    for value in self {
      result.push(string(value, length))
    }
    result
  }
}

impl<'a, K> PadLen for HashMap<K, &'a str>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn run(&self, length: usize) -> Self::Output {
    let mut result: HashMap<K, String> = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), string(value, length));
    }
    result
  }
}

impl<K> PadLen for HashMap<K, String>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn run(&self, length: usize) -> Self::Output {
    let mut result: HashMap<K, String> = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), string(value, length));
    }
    result
  }
}

fn string(string: &str, length: usize) -> String {
  let str_len: usize = string.len();
  match str_len {
    str_len if str_len > length => string.chars().take(length).collect(),
    str_len if str_len < length => format!("{}{}", string, " ".repeat(length - str_len)),
    _ => string.to_string(),
  }
}
