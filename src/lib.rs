//! # serde-iter
//!
//! [![GitHub actions](https://github.com/SOF3/serde-iter/workflows/CI/badge.svg)](https://github.com/SOF3/serde-iter/actions?query=workflow%3ACI)
//! [![crates.io](https://img.shields.io/crates/v/serde_iter.svg)](https://crates.io/crates/serde_iter)
//! [![crates.io](https://img.shields.io/crates/d/serde_iter.svg)](https://crates.io/crates/serde_iter)
//! [![docs.rs](https://docs.rs/serde_iter/badge.svg)](https://docs.rs/serde_iter)
//! [![GitHub](https://img.shields.io/github/last-commit/SOF3/serde-iter)](https://github.com/SOF3/serde-iter)
//! [![GitHub](https://img.shields.io/github/stars/SOF3/serde-iter?style=social)](https://github.com/SOF3/serde-iter)
//!
//! This crate provides serializer functions to serialize iterator types as sequences and maps.
//!
//! See the documentation in each module for details.

#![warn(
    unused_qualifications,
    variant_size_differences,
    clippy::checked_conversions,
    clippy::needless_borrow,
    clippy::shadow_unrelated,
    clippy::wrong_pub_self_convention
)]
#![deny(
    anonymous_parameters,
    bare_trait_objects,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::float_cmp_const,
    clippy::if_not_else,
    clippy::indexing_slicing,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![cfg_attr(
    debug_assertions,
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        unreachable_code,
        unused_qualifications
    )
)]
#![cfg_attr(not(debug_assertions), deny(warnings, missing_docs, clippy::dbg_macro))]

#[cfg(feature = "map")]
pub mod map;

#[cfg(feature = "seq")]
pub mod seq;

#[cfg(feature = "once")]
mod once;
#[cfg(feature = "once")]
pub use once::CloneOnce;
