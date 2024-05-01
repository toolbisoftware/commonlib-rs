// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

// TODO Automate the releases
// TODO Write in-code documentation
// TODO Write tests
// TODO Add the essential traits for the exported structs and enums

pub mod error;
#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "logger")]
pub use logger::Logger;
