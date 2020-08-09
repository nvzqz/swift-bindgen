use crate::ctx_desc::TypeContextDescriptor;
use std::{fmt, ops::Deref};
use swift_sys::ctx_desc::EnumDescriptor as RawEnumDescriptor;

/// Context descriptor for a struct type.
#[repr(transparent)]
pub struct EnumDescriptor {
    raw: RawEnumDescriptor,
}

impl Deref for EnumDescriptor {
    type Target = TypeContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<TypeContextDescriptor> for EnumDescriptor {
    #[inline]
    fn as_ref(&self) -> &TypeContextDescriptor {
        self
    }
}

unsafe impl Send for EnumDescriptor {}
unsafe impl Sync for EnumDescriptor {}

impl fmt::Debug for EnumDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format name field first to make nested output easier to follow.
        f.debug_struct("EnumDescriptor")
            .field("name", &self.name())
            .field("flags", &self.flags())
            .field("parent", self.parent())
            .field("access_function", &self.access_function())
            .field("fields", &self.fields())
            .field("num_payload_cases", &self.num_payload_cases())
            .field("num_empty_cases", &self.num_empty_cases())
            .field("payload_size_offset", &self.payload_size_offset())
            .finish()
    }
}

impl EnumDescriptor {
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
    pub const unsafe fn from_raw(raw: RawEnumDescriptor) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw type context descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawEnumDescriptor {
        self.raw
    }
}

impl EnumDescriptor {
    /// Returns the number of non-empty cases in the enum are in the low 24
    /// bits; the offset of the payload size in the metadata record in words, if
    /// any, is stored in the high 8 bits.
    #[inline]
    pub fn num_payload_cases_and_payload_size_offset(&self) -> u32 {
        self.raw.num_payload_cases_and_payload_size_offset
    }

    /// Returns the number of empty cases in the enum.
    #[inline]
    pub fn num_empty_cases(&self) -> u32 {
        self.raw.num_empty_cases
    }

    /// Returns the number of non-empty cases in the enum.
    #[inline]
    pub const fn num_payload_cases(&self) -> u32 {
        self.raw.num_payload_cases()
    }

    /// Returns the total number of cases in the enum.
    #[inline]
    pub const fn num_cases(&self) -> u32 {
        self.raw.num_cases()
    }

    /// Returns the offset of the payload size in the metadata record in words,
    /// if any.
    #[inline]
    pub const fn payload_size_offset(&self) -> u8 {
        self.raw.payload_size_offset()
    }

    // TODO: Create methods for trailing objects.
}
