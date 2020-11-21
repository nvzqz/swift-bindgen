use crate::metadata::Metadata;
use std::fmt;
use swift_sys::metadata::{TupleMetadata as RawTupleMetadata, ValueWitnessTable};

mod element;
mod labeled_element_iter;
mod labels;

pub use element::*;
pub use labeled_element_iter::*;
pub use labels::*;

/// Metadata for tuples.
#[repr(transparent)]
pub struct TupleMetadata {
    raw: RawTupleMetadata,
}

impl AsRef<Metadata> for TupleMetadata {
    #[inline]
    fn as_ref(&self) -> &Metadata {
        unsafe { &*(self as *const _ as *const _) }
    }
}

unsafe impl Send for TupleMetadata {}
unsafe impl Sync for TupleMetadata {}

impl fmt::Debug for TupleMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_struct = f.debug_struct("TupleMetadata");

        debug_struct
            .field("kind", &self.as_metadata().kind())
            .field("value_witnesses", self.value_witnesses());

        // If any elements have labels, format elements as a map with labels.
        // Otherwise, format as a slice.
        if self.has_labels() {
            struct LabeledElements<'a>(TupleMetadataLabeledElementIter<'a>);

            impl fmt::Debug for LabeledElements<'_> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.debug_map().entries(self.0.clone()).finish()
                }
            }

            debug_struct.field(
                "labeled_elements",
                &LabeledElements(self.labeled_elements()),
            );
        } else {
            debug_struct.field("elements", &self.elements());
        }

        debug_struct.finish()
    }
}

impl TupleMetadata {
    /// Creates an instance from a raw tuple metadata value.
    ///
    /// # Safety
    ///
    /// The resulting context where `self` is placed must be correct for the
    /// value of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawTupleMetadata) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw tuple metadata value.
    #[inline]
    pub const fn into_raw(self) -> RawTupleMetadata {
        self.raw
    }

    /// Returns a reference to the inner raw tuple metadata value.
    #[inline]
    pub const fn as_raw(&self) -> &RawTupleMetadata {
        &self.raw
    }
}

impl TupleMetadata {
    /// Casts the tuple metadata to a type-erased metadata.
    #[inline]
    pub fn as_metadata(&self) -> &Metadata {
        self.as_ref()
    }

    /// Returns the value-witness table.
    #[inline]
    pub fn value_witnesses(&self) -> &ValueWitnessTable {
        self.as_metadata().value_witnesses()
    }

    /// Returns `true` if any tuple element has a label.
    #[inline]
    pub fn has_labels(&self) -> bool {
        !self.raw.labels.is_null()
    }

    /// Returns the space-separated string of labels.
    #[inline]
    pub fn labels(&self) -> Option<&TupleMetadataLabels> {
        // SAFETY: The instance does not outlive `self`, and the pointer refers
        // to valid UTF-8 data if non-null.
        unsafe { TupleMetadataLabels::new(self.raw.labels) }
    }

    /// Returns a slice to the vector of metadata for tuple elements.
    #[inline]
    pub fn elements(&self) -> &[TupleMetadataElement] {
        // SAFETY: Instances of this type are followed by the correct number of
        // elements, and `TupleMetadataElement` has the same representation as
        // the raw underlying type.
        unsafe { &*(self.raw.elements() as *const _ as *const _) }
    }

    /// Returns an iterator over the tuple elements and their labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use swift_rt::metadata::TupleMetadata;
    ///
    /// let metadata: &TupleMetadata = // ...
    /// # return;
    ///
    /// for (label, element) in metadata.labeled_elements() {
    ///     println!("{:?}: {:?}", label, element);
    /// }
    /// ```
    #[inline]
    pub fn labeled_elements(&self) -> TupleMetadataLabeledElementIter {
        TupleMetadataLabeledElementIter::new(self)
    }
}
