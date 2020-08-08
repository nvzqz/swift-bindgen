//! # Raw Swift Internals
//!
//! Low-level bindings to the Swift runtime.
//!
//! Unlike `swift-rt`—which contains idiomatic bindings—this exposes the raw
//! internals of the Swift runtime. `swift-rt` crate should be preferred, unless
//! something is missing in it.
//!
//! ## Contribute
//!
//! This is a work-in-progress by [Nikolai Vazquez](https://twitter.com/nikolaivazquez).
//! If you would like to get involved,
//! <a href="mailto:hello@nikolaivazquez.com?subject=I want to help with swift-bindgen&body=Hi Nikolai,%0A%0AMy name is YOUR NAME and I want to get involved with swift-bindgen by...">reach out</a>!
//!
//! If this project is useful to you, please support it by
//! [sponsoring on GitHub](https://github.com/sponsors/nvzqz) or
//! [donating directly](https://www.paypal.me/nvzqz)!

#![warn(missing_docs)]

pub mod metadata;
pub mod ptr;
pub mod reflection;

mod opaque;

pub use opaque::*;
