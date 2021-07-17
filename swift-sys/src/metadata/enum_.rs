use crate::{ctx_desc::EnumDescriptor, metadata::Metadata};

/// Metadata for enums.
///
/// This type deliberately does not implement [`Copy`] in order to avoid
/// accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumMetadata {
    /// The base metadata.
    pub base: Metadata,

    /// An out-of-line description of the type.
    pub type_descriptor: *const EnumDescriptor,
}
