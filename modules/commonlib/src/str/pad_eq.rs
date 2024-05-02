// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{collections::HashMap, hash::Hash};

pub trait PadEq {
  type Output;
  fn run(&self) -> Self::Output;
}

pub fn pad_eq<T: PadEq>(value: T) -> T::Output {
  value.run()
}

impl<'a> PadEq for Vec<&'a str> {
  type Output = Vec<String>;
  fn run(&self) -> Self::Output {
    let length: usize = self.iter().map(|v: &&str| v.len()).max().unwrap_or(0);
    let mut result: Vec<String> = Vec::new();
    for value in self {
      result.push(run(value, length))
    }
    result
  }
}

impl PadEq for Vec<String> {
  type Output = Vec<String>;
  fn run(&self) -> Self::Output {
    let length: usize = self.iter().map(|v: &String| v.len()).max().unwrap_or(0);
    let mut result: Vec<String> = Vec::new();
    for value in self {
      result.push(run(value, length))
    }
    result
  }
}

impl<'a, K> PadEq for HashMap<K, &'a str>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn run(&self) -> Self::Output {
    let length: usize = self.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    let mut result: HashMap<K, String> = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), run(value, length));
    }
    result
  }
}

impl<K> PadEq for HashMap<K, String>
where
  K: Hash + Eq + Clone,
{
  type Output = HashMap<K, String>;
  fn run(&self) -> Self::Output {
    let length: usize = self.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    let mut result: HashMap<K, String> = HashMap::new();
    for (key, value) in self {
      result.insert(key.to_owned(), run(value, length));
    }
    result
  }
}

fn run(string: &str, length: usize) -> String {
  let str_len: usize = string.len();
  match str_len {
    str_len if str_len < length => format!("{}{}", string, " ".repeat(length - str_len)),
    _ => string.to_string(),
  }
}
