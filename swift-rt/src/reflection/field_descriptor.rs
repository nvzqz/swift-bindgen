use crate::{
    mangling::Mangled,
    reflection::{FieldDescriptorKind, FieldRecord},
};
use std::{fmt, slice};
use swift_sys::{ptr::RelativeDirectPointer, reflection::FieldDescriptor as RawFieldDescriptor};

/// A collection of field records for a single class, struct or enum
/// declaration.
#[repr(C)]
pub struct FieldDescriptor {
    raw: RawFieldDescriptor,
    field_records: [FieldRecord; 0],
}

unsafe impl Send for FieldDescriptor {}
unsafe impl Sync for FieldDescriptor {}

impl fmt::Debug for FieldDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FieldDescriptor")
            .field("type_name", &self.type_name())
            .field("superclass_name", &self.superclass_name())
            .field("kind", &self.raw.kind)
            .field("field_record_size", &self.raw.field_record_size)
            .field("num_fields", &self.raw.num_fields)
            .field("field_records", &self.field_records())
            .finish()
    }
}

impl FieldDescriptor {
    /// Creates an instance from a raw context descriptor value.
    ///
    /// # Safety
    ///
    /// - The resulting location where `self` is placed must be correct for the
    ///   fields of the raw value.
    ///
    ///   It must also be proceeded by the declared number of field records.
    ///
    /// - Invariants indicated by the field descriptor kind must be upheld.
    ///
    /// - The field record size must be correct.
    #[inline]
    pub const unsafe fn from_raw(raw: RawFieldDescriptor) -> Self {
        Self {
            raw,
            field_records: [],
        }
    }

    /// Extracts the inner field descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawFieldDescriptor {
        self.raw
    }

    /// Returns a shared reference to the inner raw field descriptor value.
    #[inline]
    pub const fn as_raw(&self) -> &RawFieldDescriptor {
        &self.raw
    }
}

impl FieldDescriptor {
    /// Returns the mangled name of the type.
    #[inline]
    pub fn type_name(&self) -> Option<&Mangled> {
        unsafe { self.type_name_ptr().as_ref() }
    }

    /// Returns a pointer to the mangled name of the type.
    #[inline]
    pub fn type_name_ptr(&self) -> &RelativeDirectPointer<Mangled> {
        self.raw.mangled_type_name.cast_by_ref()
    }

    /// Returns the mangled name of the type's superclass.
    #[inline]
    pub fn superclass_name(&self) -> Option<&Mangled> {
        unsafe { self.superclass_name_ptr().as_ref() }
    }

    /// Returns a pointer to the mangled name of the type's superclass.
    #[inline]
    pub fn superclass_name_ptr(&self) -> &RelativeDirectPointer<Mangled> {
        self.raw.superclass.cast_by_ref()
    }

    /// Returns the kind of field being described.
    #[inline]
    pub fn kind(&self) -> FieldDescriptorKind {
        self.raw.kind
    }

    /// Returns the size of a `FieldRecord`. This appears to be unchanged.
    #[inline]
    pub fn field_record_size(&self) -> u16 {
        self.raw.field_record_size
    }

    /// Returns the number of fields the type has.
    #[inline]
    pub fn num_fields(&self) -> u32 {
        self.raw.num_fields
    }

    /// Returns the records for this type's fields.
    #[inline]
    pub fn field_records(&self) -> &[FieldRecord] {
        // TODO: Investigate if field records need a dynamically-sized slice.

        let len = self.num_fields() as usize;
        let start = self.field_records.as_ptr();

        unsafe { slice::from_raw_parts(start, len) }
    }
}
