//! Swift type metadata.
//!
//! # Relevant files
//!
//! - [`Metadata.h`](https://github.com/apple/swift/blob/master/include/swift/ABI/Metadata.h)
//! - [`MetadataValues.h`](https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataValues.h)
//! - [`MetadataKind.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataKind.def)
//! - [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def)

mod context_descriptor;
mod kind;
mod metadata;
mod request;
mod response;
mod state;
mod value_witness;

pub use context_descriptor::*;
pub use kind::*;
pub use metadata::*;
pub use request::*;
pub use response::*;
pub use state::*;
pub use value_witness::*;
