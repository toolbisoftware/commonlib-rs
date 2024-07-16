// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub use self::error::{error, errorf, Error};
#[cfg(feature = "logger")]
pub use self::logger::{LogLevel, Logger};
pub use self::misc::as_variant;
pub use self::num::round;
pub use self::str::{pad_eq, pad_len, trunc};
pub use self::time::{Stopwatch, Time, TimeUnit, TimeValue, TimeValueKind};

pub mod error;
#[cfg(feature = "logger")]
pub mod logger;
pub mod misc;
pub mod num;
pub mod prelude;
pub mod str;
pub mod time;
