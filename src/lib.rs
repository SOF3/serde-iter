//! This crate provides serializer functions to serialize iterator types as sequences and maps.
//!
//! See the documentation in each module for details.

#[cfg(feature = "map")]
pub mod map;
#[cfg(feature = "seq")]
pub mod seq;
