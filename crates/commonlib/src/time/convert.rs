// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use std::{fmt::Display, time::Duration};

use thiserror::Error;

#[derive(Debug)]
pub struct TimeConvert {
  duration: Duration,
}

#[derive(Debug)]
pub struct TimeConvertUnit {
  pub kind: TimeConvertUnitKind,
  pub value: TimeConvertValueKind,
}

#[derive(Debug)]
pub enum TimeConvertUnitKind {
  Seconds,
  Milliseconds,
  Microseconds,
  Nanoseconds,
}

#[derive(Debug)]
pub enum TimeConvertValueKind {
  Integer(u32),
  Float(f32),
}

#[derive(Debug, Error)]
pub enum TimeConvertError {
  #[error("precision cannot be higher than 5 decimals")]
  InvalidPrecision,
}

impl TimeConvert {
  pub fn optimal(self) -> TimeConvertUnit {
    let (unit, value) = self.__optimal();

    TimeConvertUnit {
      kind: unit,
      value: TimeConvertValueKind::Integer(value as u32),
    }
  }

  pub fn s(self) -> TimeConvertUnit {
    let value = self.duration.as_nanos() as f32 / 1_000_000_000.0;

    TimeConvertUnit {
      kind: TimeConvertUnitKind::Seconds,
      value: TimeConvertValueKind::Integer(value as u32),
    }
  }

  pub fn ms(self) -> TimeConvertUnit {
    let value = self.duration.as_nanos() as f32 / 1_000_000.0;

    TimeConvertUnit {
      kind: TimeConvertUnitKind::Milliseconds,
      value: TimeConvertValueKind::Integer(value as u32),
    }
  }

  pub fn us(self) -> TimeConvertUnit {
    let value = self.duration.as_nanos() as f32 / 1_000.0;

    TimeConvertUnit {
      kind: TimeConvertUnitKind::Microseconds,
      value: TimeConvertValueKind::Integer(value as u32),
    }
  }

  pub fn ns(self) -> TimeConvertUnit {
    let value = self.duration.as_nanos() as f32;

    TimeConvertUnit {
      kind: TimeConvertUnitKind::Nanoseconds,
      value: TimeConvertValueKind::Integer(value as u32),
    }
  }

  pub fn optimalf(self, precision: Option<usize>) -> TimeConvertUnit {
    let (unit, value) = self.__optimal();
    let rounded = round(value, precision);

    TimeConvertUnit {
      kind: unit,
      value: TimeConvertValueKind::Float(rounded),
    }
  }

  pub fn sf(self, precision: Option<usize>) -> TimeConvertUnit {
    let value = self.duration.as_nanos() as f32 / 1_000_000_000.0;
    let rounded = round(value, precision);

    TimeConvertUnit {
      kind: TimeConvertUnitKind::Seconds,
      value: TimeConvertValueKind::Float(rounded),
    }
  }

  pub fn msf(self, precision: Option<usize>) -> TimeConvertUnit {
    let value = self.duration.as_nanos() as f32 / 1_000_000.0;
    let rounded = round(value, precision);

    TimeConvertUnit {
      kind: TimeConvertUnitKind::Milliseconds,
      value: TimeConvertValueKind::Float(rounded),
    }
  }

  pub fn usf(self, precision: Option<usize>) -> TimeConvertUnit {
    let value = self.duration.as_nanos() as f32 / 1_000.0;
    let rounded = round(value, precision);

    TimeConvertUnit {
      kind: TimeConvertUnitKind::Microseconds,
      value: TimeConvertValueKind::Float(rounded),
    }
  }

  fn __optimal(&self) -> (TimeConvertUnitKind, f32) {
    let nanos = self.duration.as_nanos();
    match nanos {
      dur if dur >= 1_000_000_000 => (TimeConvertUnitKind::Seconds, nanos as f32 / 1_000_000_000.0),
      dur if dur >= 1_000_000 => (
        TimeConvertUnitKind::Milliseconds,
        nanos as f32 / 1_000_000.0,
      ),
      dur if dur >= 1_000 => (TimeConvertUnitKind::Microseconds, nanos as f32 / 1_000.0),
      _ => (TimeConvertUnitKind::Nanoseconds, nanos as f32),
    }
  }
}

impl From<Duration> for TimeConvert {
  fn from(value: Duration) -> Self {
    Self { duration: value }
  }
}

impl Display for TimeConvertUnit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let unit = match self.kind {
      TimeConvertUnitKind::Seconds => "s",
      TimeConvertUnitKind::Milliseconds => "ms",
      TimeConvertUnitKind::Microseconds => "μs",
      TimeConvertUnitKind::Nanoseconds => "ns",
    };

    match self.value {
      TimeConvertValueKind::Integer(int) => {
        write!(f, "{}{}", int, unit)
      }
      TimeConvertValueKind::Float(float) => {
        write!(f, "{}{}", float, unit)
      }
    }
  }
}

// TODO Convert to a global function

fn round(value: f32, precision: Option<usize>) -> f32 {
  let precision = 10usize.pow(precision.unwrap_or(3) as u32);
  (value * precision as f32).round() / precision as f32
}
