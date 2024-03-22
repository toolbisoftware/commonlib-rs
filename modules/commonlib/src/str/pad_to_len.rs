// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, hash::Hash};

fn run(string: &str, length: usize) -> String {
  let str_len: usize = string.len();
  match str_len {
    str_len if str_len > length => string.chars().take(length).collect(),
    str_len if str_len < length => format!("{}{}", string, " ".repeat(length - str_len)),
    _ => string.to_string(),
  }
}

pub trait PadToLen {
  type Output;
  fn run(&self, length: usize) -> Self::Output;
}

impl PadToLen for &str {
  type Output = String;
  fn run(&self, length: usize) -> Self::Output {
    run(self, length)
  }
}

impl<'a, K> PadToLen for HashMap<K, &'a str>
where
  K: Eq + Hash + Clone,
{
  type Output = HashMap<K, String>;
  fn run(&self, length: usize) -> Self::Output {
    let mut result: HashMap<K, String> = HashMap::new();
    for (key, string) in self {
      result.insert(key.to_owned(), run(string, length));
    }

    result
  }
}

impl<'a> PadToLen for Vec<&'a str> {
  type Output = Vec<String>;
  fn run(&self, length: usize) -> Self::Output {
    let mut result: Vec<String> = Vec::new();
    for string in self {
      result.push(run(string, length));
    }

    result
  }
}

pub fn pad_to_len<T: PadToLen>(value: T, length: usize) -> T::Output {
  value.run(length)
}
