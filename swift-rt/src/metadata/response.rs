use crate::metadata::{Metadata, MetadataState};
use std::fmt;
use swift_sys::metadata::MetadataResponse as RawMetadataResponse;

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
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataResponse {
    raw: RawMetadataResponse,
}

unsafe impl Send for MetadataResponse {}
unsafe impl Sync for MetadataResponse {}

impl fmt::Debug for MetadataResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &dyn fmt::Debug = match self.completed_value() {
            Some(value) => value,
            None => {
                // The metadata is incomplete, so we cannot safely inspect its
                // current value.
                struct IncompletePlaceholder;

                impl fmt::Debug for IncompletePlaceholder {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        f.write_str("<incomplete>")
                    }
                }

                &IncompletePlaceholder
            }
        };

        f.debug_struct("MetadataResponse")
            .field("value", value)
            .field("state", &self.raw.state)
            .finish()
    }
}

impl MetadataResponse {
    /// Creates an instance from a raw metadata response value.
    ///
    /// # Safety
    ///
    /// The metadata response must be valid.
    #[inline]
    pub const unsafe fn from_raw(raw: RawMetadataResponse) -> Self {
        Self { raw }
    }

    /// Returns the raw value this value is based on.
    #[inline]
    pub const fn as_raw(&self) -> &RawMetadataResponse {
        &self.raw
    }

    /// Returns a pointer to the requested metadata.
    #[inline]
    pub fn value_ptr(&self) -> *const Metadata {
        self.raw.value.cast()
    }

    /// Returns a reference to the requested metadata.
    ///
    /// # Safety
    ///
    /// The metadata may be concurrently under construction.
    #[inline]
    pub unsafe fn value(&self) -> &'static Metadata {
        &*self.value_ptr()
    }

    /// Returns a reference to the requested metadata if it's complete.
    #[inline]
    pub fn completed_value(&self) -> Option<&'static Metadata> {
        if self.state().is_complete() {
            Some(unsafe { self.value() })
        } else {
            None
        }
    }

    /// Returns the current state of the metadata returned.
    ///
    /// Always use this instead of trying to inspect the metadata directly to
    /// see if it satisfies the request. An incomplete metadata may be getting
    /// initialized concurrently. But this can generally be ignored if the
    /// metadata request was for abstract metadata or if the request is
    /// blocking.
    #[inline]
    pub fn state(&self) -> MetadataState {
        self.raw.state
    }
}
