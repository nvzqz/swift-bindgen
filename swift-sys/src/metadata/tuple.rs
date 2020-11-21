use crate::metadata::Metadata;
use std::{os::raw::c_char, slice};

/// Metadata for tuples.
///
/// This type deliberately does not implement [`Copy`] in order to avoid
/// accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleMetadata {
    /// The base metadata.
    pub base: Metadata,

    /// The number of elements in the tuple.
    pub num_elements: usize,

    /// A null-terminated UTF-8 string string containing the tuple's labels,
    /// separated by spaces.
    ///
    /// For example, the labels in the tuple type `(x: Int, Int, z: Int)` would
    /// be encoded as `"x  z \0"`.
    ///
    /// A label (possibly zero-length) is provided for each element of the
    /// tuple, meaning that the label string for a tuple of **n** elements
    /// always contains exactly **n** spaces. If the tuple has no labels at all,
    /// the label string is a null pointer.
    pub labels: *const c_char,
}

impl TupleMetadata {
    /// Returns a pointer to the vector of metadata for tuple elements.
    #[inline]
    pub fn elements_ptr(this: *const Self) -> *const TupleMetadataElement {
        this.wrapping_add(1).cast()
    }

    /// Returns a slice to the vector of metadata for tuple elements.
    ///
    /// # Safety
    ///
    /// This instance must be followed by [`num_elements`](Self::num_elements)
    /// many [`TupleMetadataElement`] instances.
    #[inline]
    pub unsafe fn elements(&self) -> &[TupleMetadataElement] {
        slice::from_raw_parts(Self::elements_ptr(self), self.num_elements)
    }
}

/// A tuple element in [`TupleMetadata`].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleMetadataElement {
    /// The type metadata of the element.
    pub ty: *const Metadata,

    /// The offset of the tuple element within the tuple.
    ///
    /// This is a [`usize`] on Apple targets, and a [`u32`] everywhere else.
    pub offset: TupleMetadataElementOffset,
}

macro_rules! def_element_offset {
    ($(#[$meta:meta])+) => {
        $(#[$meta])+
        #[cfg(target_vendor = "apple")]
        pub type TupleMetadataElementOffset = usize;

        $(#[$meta])+
        #[cfg(not(target_vendor = "apple"))]
        pub type TupleMetadataElementOffset = u32;
    };
}

def_element_offset! {
    /// The integer scalar type for [`TupleMetadataElement::offset`].
    ///
    /// This is a [`usize`] on Apple targets, and a [`u32`] everywhere else.
}
