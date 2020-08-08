//! Swift type reflection.
//!
//! # Relevant files
//!
//! - [`Records.h`](https://github.com/apple/swift/blob/master/include/swift/Reflection/Records.h)

mod field_descriptor;
mod field_record;

pub use field_descriptor::*;
pub use field_record::*;
