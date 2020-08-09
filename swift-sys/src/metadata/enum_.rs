use crate::{ctx_desc::EnumDescriptor, metadata::Metadata};

/// Metadata for enums.
///
/// This type deliberately does not implement
/// [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) in order to
/// avoid accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumMetadata {
    /// The base metadata.
    pub base: Metadata,

    /// An out-of-line description of the type.
    pub description: *const EnumDescriptor,
}
