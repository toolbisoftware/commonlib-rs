// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::time::{Duration, Instant};

pub struct Stopwatch {
  start_time: Option<Instant>,
}

impl Stopwatch {
  pub fn new() -> Self {
    Self { start_time: None }
  }

  pub fn start(&mut self) {
    self.start_time = Some(Instant::now())
  }

  pub fn elapsed(&self) -> f64 {
    match self.start_time {
      Some(start_time) => {
        let elapsed: Duration = start_time.elapsed();
        (elapsed.as_secs_f64() * 1000.0).round() / 1000.0
      }
      None => 0.000,
    }
  }
}

pub fn stopwatch() -> Stopwatch {
  Stopwatch::new()
}
