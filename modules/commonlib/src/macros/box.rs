// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[macro_export]
macro_rules! boxn {
  ($expr:expr) => {
    Box::new($expr)
  };
}
