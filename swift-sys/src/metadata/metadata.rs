use crate::{
    ctx_desc::TypeContextDescriptor,
    metadata::{fns, MetadataKind, MetadataRequest, MetadataResponse, ValueWitnessTable},
};
use std::{ffi::c_void, ptr, slice, str};

/// Raw type metadata.
///
/// This type deliberately does not implement [`Copy`] in order to avoid
/// accidentally dereferencing from the wrong location.
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
    /// Fetch a uniqued metadata object for a generic nominal type.
    ///
    /// # Safety
    ///
    /// The arguments vector must be correct for the given type descriptor.
    #[inline]
    #[doc(alias = "swift_getGenericMetadata")]
    pub unsafe fn get_generic(
        request: MetadataRequest,
        arguments: *const *const c_void,
        description: *const TypeContextDescriptor,
    ) -> MetadataResponse {
        fns::swift_getGenericMetadata(request, arguments, description)
    }

    /// Returns the name of a Swift type represented by a metadata object.
    #[inline]
    #[doc(alias = "swift_getTypeName")]
    pub unsafe fn name(this: *const Self, qualified: bool) -> &'static str {
        let name = fns::swift_getTypeName(this, qualified);
        let slice = slice::from_raw_parts(name.data.cast::<u8>(), name.length);
        str::from_utf8_unchecked(slice)
    }

    /// Returns the mangled name of a Swift type represented by a metadata
    /// object.
    ///
    /// # Availability
    ///
    /// **Swift:** 5.3
    #[inline]
    #[doc(alias = "swift_getMangledTypeName")]
    pub unsafe fn mangled_name(this: *const Self) -> &'static str {
        // TODO: Dynamically load the symbol at runtime and return `Result` with
        // missing symbol error type.

        let name = fns::swift_getMangledTypeName(this);
        let slice = slice::from_raw_parts(name.data.cast::<u8>(), name.length);
        str::from_utf8_unchecked(slice)
    }

    /// Returns a pointer to the value-witness table pointer from the pointer
    /// to type metadata.
    #[inline]
    pub fn value_witness_table_ptr(this: *const Self) -> *const *const ValueWitnessTable {
        // The table is at a single pointer offset before the metadata.
        this.cast::<*const ValueWitnessTable>().wrapping_sub(1)
    }

    /// Returns a pointer to the type descripton from the pointer to type
    /// metadata.
    ///
    /// # Safety
    ///
    /// The raw pointer must reference valid type metadata.
    #[inline]
    #[doc(alias = "swift_getTypeContextDescriptor")]
    pub unsafe fn type_descriptor(this: *const Self) -> *const TypeContextDescriptor {
        fns::swift_getTypeContextDescriptor(this)
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
