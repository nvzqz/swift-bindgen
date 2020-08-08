//! Swift type metadata.

// Re-export basic types that don't need to be wrapped.
#[doc(no_inline)]
pub use swift_sys::metadata::{MetadataKind, MetadataState};

mod metadata;
mod response;

pub use metadata::*;
pub use response::*;
