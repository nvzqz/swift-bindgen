use crate::metadata::Metadata;
use std::{fmt, marker::PhantomData};
use swift_sys::metadata::{
    TupleMetadataElement as RawTupleMetadataElement, TupleMetadataElementOffset,
};

/// A tuple element in [`TupleMetadata`](super::TupleMetadata).
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TupleMetadataElement<'a> {
    raw: RawTupleMetadataElement,
    marker: PhantomData<&'a Metadata>,
}

unsafe impl Send for TupleMetadataElement<'_> {}
unsafe impl Sync for TupleMetadataElement<'_> {}

impl fmt::Debug for TupleMetadataElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TupleMetadataElement")
            .field("ty", self.ty())
            .field("offset", &self.raw.offset)
            .finish()
    }
}

impl TupleMetadataElement<'_> {
    /// Creates an instance from a raw tuple metadata element value.
    ///
    /// # Safety
    ///
    /// The resulting context where `self` is placed must be correct for the
    /// value of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawTupleMetadataElement) -> Self {
        Self {
            raw,
            marker: PhantomData,
        }
    }

    /// Extracts the inner raw tuple metadata element value.
    #[inline]
    pub const fn into_raw(self) -> RawTupleMetadataElement {
        self.raw
    }

    /// Returns a reference to the inner raw tuple metadata element value.
    #[inline]
    pub const fn as_raw(&self) -> &RawTupleMetadataElement {
        &self.raw
    }
}

impl<'a> TupleMetadataElement<'a> {
    /// The type metadata of the element.
    #[inline]
    pub fn ty(&self) -> &'a Metadata {
        unsafe { &*self.raw.ty.cast() }
    }

    /// The offset of the tuple element within the tuple.
    ///
    /// This is a [`usize`] on Apple targets, and a [`u32`] everywhere else.
    #[inline]
    pub fn offset(&self) -> TupleMetadataElementOffset {
        self.raw.offset
    }
}
