use crate::ctx_desc::TypeContextDescriptor;
use std::{fmt, ops::Deref};
use swift_sys::ctx_desc::StructDescriptor as RawStructDescriptor;

/// Context descriptor for a struct type.
#[repr(transparent)]
pub struct StructDescriptor {
    raw: RawStructDescriptor,
}

impl Deref for StructDescriptor {
    type Target = TypeContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<TypeContextDescriptor> for StructDescriptor {
    #[inline]
    fn as_ref(&self) -> &TypeContextDescriptor {
        self
    }
}

unsafe impl Send for StructDescriptor {}
unsafe impl Sync for StructDescriptor {}

impl fmt::Debug for StructDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format name field first to make nested output easier to follow.
        f.debug_struct("StructDescriptor")
            .field("name", &self.name())
            .field("flags", &self.flags())
            .field("parent", self.parent())
            .field("access_function", &self.access_function())
            .field("fields", &self.fields())
            .field("num_fields", &self.num_fields())
            .field(
                "field_offset_vector_offset",
                &self.field_offset_vector_offset(),
            )
            .finish()
    }
}

impl StructDescriptor {
    /// Creates an instance from a raw type context descriptor value.
    ///
    /// # Safety
    ///
    /// - The resulting location where `self` is placed must be correct for the
    ///   fields of the raw value.
    ///
    /// - Invariants indicated by the context descriptor flags must be upheld.
    ///   For example, if they indicate extra fields, those must exist relative
    ///   to the resulting location.
    #[inline]
    pub const unsafe fn from_raw(raw: RawStructDescriptor) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw type context descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawStructDescriptor {
        self.raw
    }
}

impl StructDescriptor {
    /// Returns the number of stored properties in the struct. If there is a
    /// field offset vector, this is its length.
    #[inline]
    pub fn num_fields(&self) -> u32 {
        self.raw.num_fields
    }

    /// Returns the offset of the field offset vector for this struct's stored
    /// properties in its metadata, if any. 0 means there is no field offset
    /// vector.
    #[inline]
    pub fn field_offset_vector_offset(&self) -> u32 {
        self.raw.field_offset_vector_offset
    }

    /// Returns `true` if metadata records for this type have a field offset
    /// vector for its stored properties.
    #[inline]
    pub fn has_field_offset_vector(&self) -> bool {
        self.raw.has_field_offset_vector()
    }

    // TODO: Create methods for trailing objects.
}
