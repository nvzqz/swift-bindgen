use crate::{
    ctx_desc::StructDescriptor,
    metadata::{Metadata, MetadataKind},
};
use std::fmt;
use swift_sys::metadata::{StructMetadata as RawStructMetadata, ValueWitnessTable};

/// Metadata for structs.
#[repr(transparent)]
pub struct StructMetadata {
    raw: RawStructMetadata,
}

impl AsRef<Metadata> for StructMetadata {
    #[inline]
    fn as_ref(&self) -> &Metadata {
        unsafe { &*(self as *const _ as *const _) }
    }
}

unsafe impl Send for StructMetadata {}
unsafe impl Sync for StructMetadata {}

impl fmt::Debug for StructMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StructMetadata")
            .field("kind", &self.as_metadata().kind())
            .field("value_witnesses", self.value_witnesses())
            .field("type_descriptor", self.type_descriptor())
            .finish()
    }
}

impl StructMetadata {
    /// Creates an instance from a raw struct metadata value.
    ///
    /// # Safety
    ///
    /// The resulting context where `self` is placed must be correct for the
    /// value of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawStructMetadata) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw struct metadata value.
    #[inline]
    pub const fn into_raw(self) -> RawStructMetadata {
        self.raw
    }

    /// Returns a reference to the inner raw struct metadata value.
    #[inline]
    pub const fn as_raw(&self) -> &RawStructMetadata {
        &self.raw
    }
}

impl StructMetadata {
    /// Creates a new struct metadata.
    ///
    /// # Safety
    ///
    /// The metadata context must have a memory layout appropriate for the type
    /// of metadata indicated by `kind`. This includes the value-witness table
    /// that is placed immediately before the created instance.
    #[inline]
    pub const unsafe fn new(type_descriptor: *const StructDescriptor) -> Self {
        Self {
            raw: RawStructMetadata {
                base: Metadata::new(MetadataKind::STRUCT.value() as usize).into_raw(),
                type_descriptor: type_descriptor.cast(),
            },
        }
    }

    /// Casts the struct metadata to a type-erased metadata.
    #[inline]
    pub fn as_metadata(&self) -> &Metadata {
        self.as_ref()
    }

    /// Returns the value-witness table.
    #[inline]
    pub fn value_witnesses(&self) -> &ValueWitnessTable {
        self.as_metadata().value_witnesses()
    }

    /// Returns an out-of-line description of the type.
    #[inline]
    pub fn type_descriptor(&self) -> &StructDescriptor {
        unsafe { &*self.raw.type_descriptor.cast() }
    }
}
