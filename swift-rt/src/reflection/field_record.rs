use crate::{mangling::Mangled, reflection::FieldRecordFlags};
use std::{fmt, os::raw::c_char};
use swift_sys::{ptr::RelativeDirectPointer, reflection::FieldRecord as RawFieldRecord};

/// An entry for a type's field.
#[repr(transparent)]
pub struct FieldRecord {
    raw: RawFieldRecord,
}

unsafe impl Send for FieldRecord {}
unsafe impl Sync for FieldRecord {}

impl FieldRecord {
    /// Creates an instance from a raw context record value.
    ///
    /// # Safety
    ///
    /// The resulting location where `self` is placed must be correct for the
    /// fields of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawFieldRecord) -> Self {
        Self { raw }
    }

    /// Extracts the inner field record value.
    #[inline]
    pub const fn into_raw(self) -> RawFieldRecord {
        self.raw
    }

    /// Returns a shared reference to the inner raw field record value.
    #[inline]
    pub const fn as_raw(&self) -> &RawFieldRecord {
        &self.raw
    }
}

impl fmt::Debug for FieldRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FieldRecord")
            .field("flags", &self.raw.flags)
            .field("type_name", &self.type_name())
            .field("field_name", &self.field_name())
            .finish()
    }
}

impl FieldRecord {
    /// Returns the flags defining the semantics of the field.
    #[inline]
    pub fn flags(&self) -> FieldRecordFlags {
        self.raw.flags
    }

    /// Returns the mangled name of the field's type.
    #[inline]
    pub fn type_name(&self) -> Option<&Mangled> {
        unsafe { self.type_name_ptr().as_ref() }
    }

    /// Returns a pointer to the mangled name of the field's type.
    #[inline]
    pub fn type_name_ptr(&self) -> &RelativeDirectPointer<Mangled> {
        self.raw.mangled_type_name.cast_by_ref()
    }

    /// Returns the name of the field.
    #[inline]
    pub fn field_name(&self) -> Option<&str> {
        unsafe { self.field_name_ptr().as_str() }
    }

    /// Returns a pointer to the name of the field.
    #[inline]
    pub fn field_name_ptr(&self) -> &RelativeDirectPointer<c_char> {
        &self.raw.field_name
    }
}
