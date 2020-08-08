use crate::metadata::{Metadata, MetadataState};

/// The result of requesting type metadata.
///
/// This is generally the return value of a function.
///
/// For performance and ABI matching across Swift/C++, functions returning
/// this type must use `SWIFT_CC` so that the components are returned as
/// separate values.
///
/// Note that Rust currently does not support the Swift calling convention
/// (`swiftcall`), so care must be taken to ensure such functions return this
/// value correctly.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataResponse {
    /// The requested metadata.
    pub value: *const Metadata,

    /// The current state of the metadata returned.
    ///
    /// Always use this instead of trying to inspect the metadata directly to
    /// see if it satisfies the request. An incomplete metadata may be getting
    /// initialized concurrently. But this can generally be ignored if the
    /// metadata request was for abstract metadata or if the request is
    /// blocking.
    pub state: MetadataState,
}
