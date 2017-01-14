//! A module for loading and parsing Java ClassFiles (`.class`)
//!
//! This module represents the ClassFiles as a Rust struct,
//! from which you can access all the data given.
pub mod class;
pub mod info;

pub use self::class::Class;
