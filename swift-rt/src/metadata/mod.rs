//! Swift type metadata.

// Re-export basic types that don't need to be wrapped.
#[doc(no_inline)]
pub use swift_sys::metadata::{MetadataKind, MetadataState};

mod context_descriptor;
mod metadata;
mod response;

pub use context_descriptor::*;
pub use metadata::*;
pub use response::*;
