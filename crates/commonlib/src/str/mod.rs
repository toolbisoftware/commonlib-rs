// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

pub use self::pad_eq::{pad_eq, PadEq};
pub use self::pad_len::{pad_len, PadLen};
pub use self::trunc::{trunc, Trunc};

mod pad_eq;
mod pad_len;
mod trunc;
