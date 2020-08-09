use crate::metadata::Metadata;

/// Metadata for metatypes.
///
/// This type deliberately does not implement
/// [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) in order to
/// avoid accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetatypeMetadata {
    /// The base metadata.
    pub base: Metadata,

    /// The type metadata.
    pub instance_type: *const Metadata,
}
