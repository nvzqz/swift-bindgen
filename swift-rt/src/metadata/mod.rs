//! Swift type metadata.

// Re-export basic types that don't need to be wrapped.
#[doc(no_inline)]
pub use swift_sys::metadata::{MetadataKind, MetadataState};

mod enum_;
mod metadata;
mod metatype;
mod response;

pub use enum_::*;
pub use metadata::*;
pub use metatype::*;
pub use response::*;
