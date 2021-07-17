use crate::{ctx_desc::StructDescriptor, metadata::Metadata};

/// Metadata for structs.
///
/// This type deliberately does not implement [`Copy`] in order to avoid
/// accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructMetadata {
    /// The base metadata.
    pub base: Metadata,

    /// An out-of-line description of the type.
    pub type_descriptor: *const StructDescriptor,
}
