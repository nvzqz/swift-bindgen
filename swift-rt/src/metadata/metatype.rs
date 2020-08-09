use crate::metadata::{Metadata, MetadataKind};
use std::fmt;
use swift_sys::metadata::{MetatypeMetadata as RawMetatypeMetadata, ValueWitnessTable};

/// Metadata for metatypes.
#[repr(transparent)]
pub struct MetatypeMetadata {
    raw: RawMetatypeMetadata,
}

impl AsRef<Metadata> for MetatypeMetadata {
    #[inline]
    fn as_ref(&self) -> &Metadata {
        unsafe { &*(self as *const _ as *const _) }
    }
}

unsafe impl Send for MetatypeMetadata {}
unsafe impl Sync for MetatypeMetadata {}

impl fmt::Debug for MetatypeMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetatypeMetadata")
            .field("instance_type", self.instance_type())
            .field("value_witnesses", self.value_witnesses())
            .finish()
    }
}

impl MetatypeMetadata {
    /// Creates an instance from a raw metatype metadata value.
    ///
    /// # Safety
    ///
    /// The resulting context where `self` is placed must be correct for the
    /// value of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawMetatypeMetadata) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw metatype metadata value.
    #[inline]
    pub const fn into_raw(self) -> RawMetatypeMetadata {
        self.raw
    }

    /// Returns a reference to the inner raw metatype metadata value.
    #[inline]
    pub const fn as_raw(&self) -> &RawMetatypeMetadata {
        &self.raw
    }
}

impl MetatypeMetadata {
    /// Creates a new metatype metadata.
    ///
    /// # Safety
    ///
    /// The metadata context must have a memory layout appropriate for the type
    /// of metadata indicated by `kind`. This includes the value-witness table
    /// that is placed immediately before the created instance.
    #[inline]
    pub const unsafe fn new(instance_type: *const Metadata) -> Self {
        Self {
            raw: RawMetatypeMetadata {
                base: Metadata::new(MetadataKind::METATYPE.value() as usize).into_raw(),
                instance_type: instance_type.cast(),
            },
        }
    }

    /// Casts the metatype metadata to a type-erased metadata.
    #[inline]
    pub fn as_metadata(&self) -> &Metadata {
        self.as_ref()
    }

    /// Returns the type metadata.
    #[inline]
    pub fn instance_type(&self) -> &Metadata {
        unsafe { &*self.raw.instance_type.cast() }
    }

    /// Returns the value-witness table.
    #[inline]
    pub fn value_witnesses(&self) -> &ValueWitnessTable {
        self.as_metadata().value_witnesses()
    }
}
