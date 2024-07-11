// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use num_traits::Float;

pub trait Round {
  fn round_dec(self, decimals: usize) -> Self;
}

pub fn round<T: Round>(value: T, decimals: usize) -> T {
  value.round_dec(decimals)
}

impl<T: Float> Round for T {
  fn round_dec(self, decimals: usize) -> Self {
    let factor = T::from(10).unwrap().powi(decimals as i32);

    (self * factor).round() / factor
  }
}
