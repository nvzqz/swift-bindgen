//! Context descriptor types.

// Re-export basic types that don't need to be wrapped.
#[doc(no_inline)]
pub use swift_sys::ctx_desc::{ContextDescriptorFlags, ContextDescriptorKind};

mod base;
mod extension;
mod module;
mod protocol;
mod ty;

pub use base::*;
pub use extension::*;
pub use module::*;
pub use protocol::*;
pub use ty::*;
