use crate::{
    ctx_desc::EnumDescriptor,
    metadata::{Metadata, MetadataKind},
};
use std::fmt;
use swift_sys::metadata::{EnumMetadata as RawEnumMetadata, EnumValueWitnessTable};

/// Metadata for enums.
#[repr(transparent)]
pub struct EnumMetadata {
    raw: RawEnumMetadata,
}

impl AsRef<Metadata> for EnumMetadata {
    #[inline]
    fn as_ref(&self) -> &Metadata {
        unsafe { &*(self as *const _ as *const _) }
    }
}

unsafe impl Send for EnumMetadata {}
unsafe impl Sync for EnumMetadata {}

impl fmt::Debug for EnumMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumMetadata")
            .field("kind", &self.as_metadata().kind())
            .field("value_witnesses", self.value_witnesses())
            .field("description", self.description())
            .finish()
    }
}

impl EnumMetadata {
    /// Creates an instance from a raw enum metadata value.
    ///
    /// # Safety
    ///
    /// The resulting context where `self` is placed must be correct for the
    /// value of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawEnumMetadata) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw enum metadata value.
    #[inline]
    pub const fn into_raw(self) -> RawEnumMetadata {
        self.raw
    }

    /// Returns a reference to the inner raw enum metadata value.
    #[inline]
    pub const fn as_raw(&self) -> &RawEnumMetadata {
        &self.raw
    }
}

impl EnumMetadata {
    /// Creates a new enum metadata.
    ///
    /// # Safety
    ///
    /// The metadata context must have a memory layout appropriate for the type
    /// of metadata indicated by `kind`. This includes the value-witness table
    /// that is placed immediately before the created instance.
    #[inline]
    pub const unsafe fn new(description: *const EnumDescriptor) -> Self {
        Self {
            raw: RawEnumMetadata {
                base: Metadata::new(MetadataKind::ENUM.value() as usize).into_raw(),
                description: description.cast(),
            },
        }
    }

    /// Casts the enum metadata to a type-erased metadata.
    #[inline]
    pub fn as_metadata(&self) -> &Metadata {
        self.as_ref()
    }

    #[inline]
    pub(crate) fn value_witness_table_ptr(
        this: *const Self,
    ) -> *const *const EnumValueWitnessTable {
        Metadata::value_witness_table_ptr(this.cast()).cast()
    }

    /// Returns the enum value-witness table.
    #[inline]
    pub fn value_witnesses(&self) -> &EnumValueWitnessTable {
        debug_assert!(
            self.as_metadata()
                .value_witnesses()
                .flags
                .has_enum_witnesses(),
            "missing enum value witnesses for {:?} enum metadata",
            self.description().name(),
        );

        unsafe { &**Self::value_witness_table_ptr(self) }
    }

    /// Returns an out-of-line description of the type.
    #[inline]
    pub fn description(&self) -> &EnumDescriptor {
        unsafe { &*self.raw.description.cast() }
    }
}
