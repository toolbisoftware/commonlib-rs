// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::time::{Duration, Instant};

pub struct Stopwatch {
  start_time: Option<Instant>,
}

impl Stopwatch {
  pub fn new() -> Self {
    Stopwatch { start_time: None }
  }

  pub fn start(&mut self) {
    self.start_time = Some(Instant::now())
  }

  pub fn get_elapsed(&self) -> Option<f64> {
    if let None = self.start_time {
      return None;
    }

    let elapsed: Duration = self.start_time.unwrap().elapsed();
    let elapsed: u64 = elapsed.as_secs() * 1000 + u64::from(elapsed.subsec_millis());
    let elapsed: f64 = elapsed as f64 / 1000.0;

    Some(elapsed)
  }
}
