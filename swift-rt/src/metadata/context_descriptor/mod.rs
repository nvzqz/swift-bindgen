// Re-export basic types that don't need to be wrapped.
#[doc(no_inline)]
pub use swift_sys::metadata::{ContextDescriptorFlags, ContextDescriptorKind};

mod base;
mod module;

pub use base::*;
pub use module::*;
