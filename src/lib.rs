//! # Tweers
//! An easy-to-use Rust library for accessing the Twitter API.
//!
//! This library uses Twitter API v2 and OAuth 1.0a and provides REST APIs.
mod client;
mod v2;

pub use client::*;
pub use v2::V2;
