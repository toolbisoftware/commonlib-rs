// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{marker::PhantomData, time::Instant};

use crate::{Time, TimeValue};

#[derive(Debug)]
pub struct Stopwatch<State = Stopped> {
  start_time: Option<Instant>,
  _state: PhantomData<State>,
}

pub struct Stopped;

pub struct Running;

impl Stopwatch {
  pub fn new() -> Self {
    Self {
      start_time: None,
      _state: PhantomData::<Stopped>,
    }
  }
}

impl Stopwatch<Stopped> {
  pub fn start(self) -> Stopwatch<Running> {
    Stopwatch {
      start_time: Some(Instant::now()),
      _state: PhantomData::<Running>,
    }
  }
}

impl Stopwatch<Running> {
  pub fn elapsed(&self) -> TimeValue {
    let time = self.elapsed_to_time();

    time.optimal()
  }

  pub fn elapsed_s(&self) -> TimeValue {
    let time = self.elapsed_to_time();

    time.s()
  }

  pub fn elapsed_ms(&self) -> TimeValue {
    let time = self.elapsed_to_time();

    time.ms()
  }

  pub fn elapsed_us(&self) -> TimeValue {
    let time = self.elapsed_to_time();

    time.us()
  }

  pub fn elapsed_ns(&self) -> TimeValue {
    let time = self.elapsed_to_time();

    time.ns()
  }

  pub fn elapsedf(&self, decimals: usize) -> TimeValue {
    let time = self.elapsed_to_time();

    time.optimalf(decimals)
  }

  pub fn elapsedf_s(&self, decimals: usize) -> TimeValue {
    let time = self.elapsed_to_time();

    time.sf(decimals)
  }

  pub fn elapsedf_ms(&self, decimals: usize) -> TimeValue {
    let time = self.elapsed_to_time();

    time.msf(decimals)
  }

  pub fn elapsedf_us(&self, decimals: usize) -> TimeValue {
    let time = self.elapsed_to_time();

    time.usf(decimals)
  }

  pub fn stop(self) {
    drop(self)
  }

  fn elapsed_to_time(&self) -> Time {
    let value = self.start_time.unwrap().elapsed();
    let time = Time::from(value);

    time
  }
}
