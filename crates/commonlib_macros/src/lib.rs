// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[macro_export]
macro_rules! as_variant {
  ($value:expr, $variant:path) => {
    match $value {
      $variant(x) => Some(x),
      _ => None,
    }
  };
}
