// Re-export basic types that don't need to be wrapped.
#[doc(no_inline)]
pub use swift_sys::metadata::{ContextDescriptorFlags, ContextDescriptorKind};

mod base;
mod module;
mod ty;

pub use base::*;
pub use module::*;
pub use ty::*;
