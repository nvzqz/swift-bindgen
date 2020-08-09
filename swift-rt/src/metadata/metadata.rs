use crate::{ctx_desc::TypeContextDescriptor, metadata::MetadataKind};
use std::{ffi::c_void, fmt};
use swift_sys::metadata::{EnumValueWitnessTable, Metadata as RawMetadata, ValueWitnessTable};

/// Type metadata.
#[repr(C)]
pub struct Metadata {
    raw: RawMetadata,
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Dynamically format as the appropriate type of metadata.
        f.debug_struct("Metadata").finish()
    }
}

impl Metadata {
    /// Creates an instance from a raw metadata value.
    ///
    /// # Safety
    ///
    /// The resulting context where `self` is placed must be correct for the
    /// value of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawMetadata) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw metadata value.
    #[inline]
    pub const fn into_raw(self) -> RawMetadata {
        self.raw
    }

    /// Returns a reference to the inner raw metadata value.
    #[inline]
    pub const fn as_raw(&self) -> &RawMetadata {
        &self.raw
    }
}

impl Metadata {
    /// Creates a new metadata.
    ///
    /// # Safety
    ///
    /// The metadata context must have a memory layout appropriate for the type
    /// of metadata indicated by `kind`. This includes data that is placed
    /// immediately after or before the created instance.
    #[inline]
    pub const unsafe fn new(kind: usize) -> Self {
        Self {
            raw: RawMetadata { kind },
        }
    }

    /// Returns the kind of this metadata.
    #[inline]
    pub fn kind(&self) -> MetadataKind {
        self.raw.kind()
    }

    /// Returns the stored class isa pointer, or null if there isn't one.
    #[inline]
    pub fn isa_ptr(&self) -> *const c_void {
        self.raw.isa_ptr()
    }

    /// Returns a pointer to the value-witness table pointer from the pointer
    /// metadata.
    #[inline]
    fn value_witness_table_ptr(this: *const Self) -> *const *const ValueWitnessTable {
        RawMetadata::value_witness_table_ptr(this.cast()).cast()
    }

    /// Returns the value-witness table.
    #[inline]
    pub fn value_witnesses(&self) -> &ValueWitnessTable {
        unsafe { &**Self::value_witness_table_ptr(self) }
    }

    /// Returns the enum value-witness table if this metadata has enum witnesses
    /// one.
    #[inline]
    pub fn enum_value_witnesses(&self) -> Option<&EnumValueWitnessTable> {
        let ptr = Self::value_witness_table_ptr(self);

        let table_ptr = unsafe { *ptr };
        let table = unsafe { (*ptr).as_ref()? };

        if table.flags.has_enum_witnesses() {
            Some(unsafe { &*table_ptr.cast::<EnumValueWitnessTable>() })
        } else {
            None
        }
    }

    /// Returns a pointer to the type descriptor pointer from the pointer
    /// metadata.
    #[inline]
    fn type_descriptor_ptr(this: *const Self) -> *const *const TypeContextDescriptor {
        RawMetadata::type_descriptor_ptr(this.cast()).cast()
    }

    /// Returns a reference to the nominal type descriptor if this metadata
    /// represents a nominal type.
    #[inline]
    pub fn type_descriptor(&self) -> Option<&TypeContextDescriptor> {
        if self.kind().is_nominal_type() {
            unsafe { (*Self::type_descriptor_ptr(self)).as_ref() }
        } else {
            None
        }
    }
}
