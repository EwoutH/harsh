#![cfg_attr(feature="bench", feature(test))]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[cfg(feature = "bench")]
extern crate test;

#[cfg(test)]
mod tests;

mod error;
mod harsh;

pub use error::{Error, Result};
pub use harsh::{Harsh, HarshFactory};
