use crate::metadata::{MetadataKind, ValueWitnessTable};
use std::{ffi::c_void, ptr};

/// Raw type metadata.
///
/// This type deliberately does not implement
/// [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) in order to
/// avoid accidentally dereferencing from the wrong location.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Metadata {
    // Of type `StoredPointer`, which is a `u32` or `u64` based on target arch.
    /// The internal metadata kind value.
    ///
    /// Use the `kind` and `isa_ptr` methods to interpret this value.
    pub kind: usize,
}

impl Metadata {
    /// Returns a pointer to the value-witness table pointer from the pointer
    /// to type metadata.
    #[inline]
    pub fn value_witness_table_ptr(this: *const Self) -> *const *const ValueWitnessTable {
        // The table is at a single pointer offset before the metadata.
        this.cast::<*const ValueWitnessTable>().wrapping_sub(1)
    }

    /// Returns the kind of this metadata.
    #[inline]
    pub fn kind(&self) -> MetadataKind {
        if self.kind <= MetadataKind::LAST.value() as usize {
            unsafe { MetadataKind::new_unchecked(self.kind as u32) }
        } else {
            MetadataKind::CLASS
        }
    }

    /// Returns the stored class isa pointer, or null if there isn't one.
    #[inline]
    pub fn isa_ptr(&self) -> *const c_void {
        if self.kind > MetadataKind::LAST.value() as usize {
            self.kind as *const c_void
        } else {
            ptr::null()
        }
    }
}
