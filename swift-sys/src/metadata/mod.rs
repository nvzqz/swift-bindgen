//! Swift type metadata.
//!
//! # Relevant files
//!
//! - [`Metadata.h`](https://github.com/apple/swift/blob/master/include/swift/ABI/Metadata.h)
//! - [`MetadataValues.h`](https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataValues.h)
//! - [`MetadataKind.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataKind.def)
//! - [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def)

pub mod fns;

mod access_function;
mod enum_;
mod kind;
mod metadata;
mod metatype;
mod request;
mod response;
mod state;
mod struct_;
mod tuple;
mod value_witness;

pub use access_function::*;
pub use enum_::*;
pub use kind::*;
pub use metadata::*;
pub use metatype::*;
pub use request::*;
pub use response::*;
pub use state::*;
pub use struct_::*;
pub use tuple::*;
pub use value_witness::*;
