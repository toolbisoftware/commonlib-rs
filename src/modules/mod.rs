// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[cfg(feature = "logger")]
pub mod logger;
mod pad_string_to_length;
mod pad_strings_to_length;
mod pad_strings_to_longest;
mod stopwatch;

#[cfg(feature = "logger")]
pub use logger::Logger;
pub use pad_string_to_length::pad_string_to_length;
pub use pad_strings_to_length::pad_strings_to_length;
pub use pad_strings_to_longest::pad_strings_to_longest;
pub use stopwatch::Stopwatch;
