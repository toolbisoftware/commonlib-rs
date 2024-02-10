// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

#[test]
pub fn run() {
  let init = crate::Logger::init(Some("trace"));

  log::error!("Testing the logger.");
  log::warn!("Testing the logger.");
  log::info!("Testing the logger.");
  log::debug!("Testing the logger.");
  log::trace!("Testing the logger.");

  assert!(init.is_ok());
}
