use crate::{ctx_desc::ContextDescriptorFlags, ptr::RelativeIndirectablePointer};

/// Base class for all context descriptors.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ContextDescriptor {
    /// Flags describing the context, including its kind and format version.
    pub flags: ContextDescriptorFlags,

    /// The parent context, or null if this is a top-level context.
    pub parent: RelativeIndirectablePointer<ContextDescriptor>,
}
