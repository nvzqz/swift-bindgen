use crate::{ctx_desc::StructDescriptor, metadata::Metadata};

/// Metadata for structs.
///
/// This type deliberately does not implement
/// [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) in order to
/// avoid accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructMetadata {
    /// The base metadata.
    pub base: Metadata,

    /// An out-of-line description of the type.
    pub description: *const StructDescriptor,
}
