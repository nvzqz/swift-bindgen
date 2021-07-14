//! Swift type metadata.

// Re-export basic types that don't need to be wrapped.
#[doc(no_inline)]
pub use swift_sys::metadata::{MetadataKind, MetadataState};

mod enum_;
mod metadata;
mod metatype;
mod response;
mod struct_;
mod tuple;
mod ty;

pub use enum_::*;
pub use metadata::*;
pub use metatype::*;
pub use response::*;
pub use struct_::*;
pub use tuple::*;
pub use ty::*;
