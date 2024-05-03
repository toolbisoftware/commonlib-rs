// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[macro_export]
macro_rules! error {
  ($msg:expr $(, cat: $cat:expr)? $(, err: $err:expr)?) => {{
    let mut builder = crate::Error::new($msg);
    $(builder = builder.category($cat);)?
    $(builder = builder.error(Box::new($err));)?
    builder
  }};
}
