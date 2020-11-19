use crate::metadata::Metadata;

/// Metadata for metatypes.
///
/// This type deliberately does not implement [`Copy`] in order to avoid
/// accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetatypeMetadata {
    /// The base metadata.
    pub base: Metadata,

    /// The type metadata.
    pub instance_type: *const Metadata,
}
