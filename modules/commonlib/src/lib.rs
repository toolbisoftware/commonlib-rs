// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Automate the releases
// TODO Write in-code documentation
// TODO Write tests

pub mod error;
#[cfg(feature = "logger")]
mod logger;
mod stopwatch;
pub mod str;

#[cfg(feature = "logger")]
pub use logger::Logger;
pub use stopwatch::{stopwatch, Stopwatch};
