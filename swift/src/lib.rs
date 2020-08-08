//! # Swift Standard Library
//!
//! High-level bindings to the Swift standard library.
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

pub use swift_rt as rt;
pub use swift_sys as sys;

pub(crate) mod util;

mod never;
mod primitive;
mod ptr;

pub use never::*;
pub use primitive::*;
pub use ptr::*;
