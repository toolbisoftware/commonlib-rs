// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::time::Duration;

use crate::prelude::Round;

pub use self::stopwatch::Stopwatch;

pub mod stopwatch;

#[derive(Debug)]
pub struct Time {
  duration: Duration,
}

#[derive(Debug)]
pub struct TimeValue {
  pub unit: TimeUnit,
  pub value: TimeValueKind,
}

#[derive(Debug)]
pub enum TimeUnit {
  Seconds,
  Milliseconds,
  Microseconds,
  Nanoseconds,
}

#[derive(Debug)]
pub enum TimeValueKind {
  Integer(u32),
  Float(f32),
}

impl Time {
  pub fn optimal(&self) -> TimeValue {
    let (unit, value) = self.optimal_();

    TimeValue::from_u32(unit, value as u32)
  }

  pub fn s(&self) -> TimeValue {
    let unit = TimeUnit::Seconds;
    let value = self.duration.as_nanos() / 1_000_000_000;

    TimeValue::from_u32(unit, value as u32)
  }

  pub fn ms(&self) -> TimeValue {
    let unit = TimeUnit::Milliseconds;
    let value = self.duration.as_nanos() / 1_000_000;

    TimeValue::from_u32(unit, value as u32)
  }

  pub fn us(&self) -> TimeValue {
    let unit = TimeUnit::Microseconds;
    let value = self.duration.as_nanos() / 1_000;

    TimeValue::from_u32(unit, value as u32)
  }

  pub fn ns(&self) -> TimeValue {
    let unit = TimeUnit::Microseconds;
    let value = self.duration.as_nanos();

    TimeValue::from_u32(unit, value as u32)
  }

  pub fn optimalf(&self, decimals: usize) -> TimeValue {
    let (unit, value) = self.optimal_();
    let rounded = value.round_dec(decimals);

    TimeValue::from_f32(unit, rounded)
  }

  pub fn sf(&self, decimals: usize) -> TimeValue {
    let unit = TimeUnit::Seconds;
    let value = self.duration.as_nanos() as f32 / 1_000_000_000.0;
    let rounded = value.round_dec(decimals);

    TimeValue::from_f32(unit, rounded)
  }

  pub fn msf(&self, decimals: usize) -> TimeValue {
    let unit = TimeUnit::Milliseconds;
    let value = self.duration.as_nanos() as f32 / 1_000_000.0;
    let rounded = value.round_dec(decimals);

    TimeValue::from_f32(unit, rounded)
  }

  pub fn usf(&self, decimals: usize) -> TimeValue {
    let unit = TimeUnit::Microseconds;
    let value = self.duration.as_nanos() as f32 / 1_000.0;
    let rounded = value.round_dec(decimals);

    TimeValue::from_f32(unit, rounded)
  }

  fn optimal_(&self) -> (TimeUnit, f32) {
    let nanos = self.duration.as_nanos();

    match nanos {
      dur if dur >= 1_000_000_000 => (TimeUnit::Seconds, nanos as f32 / 1_000_000_000.0),
      dur if dur >= 1_000_000 => (TimeUnit::Milliseconds, nanos as f32 / 1_000_000.0),
      dur if dur >= 1_000 => (TimeUnit::Microseconds, nanos as f32 / 1_000.0),
      _ => (TimeUnit::Microseconds, nanos as f32),
    }
  }
}

impl From<Duration> for Time {
  fn from(value: Duration) -> Self {
    Self { duration: value }
  }
}

impl TimeValue {
  pub(crate) fn from_u32(unit: TimeUnit, value: u32) -> Self {
    Self {
      unit,
      value: TimeValueKind::Integer(value),
    }
  }

  pub(crate) fn from_f32(unit: TimeUnit, value: f32) -> Self {
    Self {
      unit,
      value: TimeValueKind::Float(value),
    }
  }
}

impl std::fmt::Display for TimeValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let symbol = match self.unit {
      TimeUnit::Seconds => "s",
      TimeUnit::Milliseconds => "ms",
      TimeUnit::Microseconds => "μs",
      TimeUnit::Nanoseconds => "ns",
    };

    match self.value {
      TimeValueKind::Integer(int) => write!(f, "{}{}", int, symbol),
      TimeValueKind::Float(float) => write!(f, "{}{}", float, symbol),
    }
  }
}
