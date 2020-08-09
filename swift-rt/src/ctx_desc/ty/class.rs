use crate::{ctx_desc::TypeContextDescriptor, mangling::Mangled};
use std::{fmt, ops::Deref};
use swift_sys::{ctx_desc::ClassDescriptor as RawClassDescriptor, ptr::RelativeDirectPointer};

/// Context descriptor for a class type.
#[repr(transparent)]
pub struct ClassDescriptor {
    raw: RawClassDescriptor,
}

impl Deref for ClassDescriptor {
    type Target = TypeContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<TypeContextDescriptor> for ClassDescriptor {
    #[inline]
    fn as_ref(&self) -> &TypeContextDescriptor {
        self
    }
}

unsafe impl Send for ClassDescriptor {}
unsafe impl Sync for ClassDescriptor {}

impl fmt::Debug for ClassDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format name field first to make nested output easier to follow.
        f.debug_struct("ClassDescriptor")
            .field("name", &self.name())
            .field(
                // TODO: Format class-specific flags as part of this property.
                "flags",
                &self.flags(),
            )
            .field("parent", self.parent())
            .field("access_function", &self.access_function())
            .field("fields", &self.fields())
            .field("superclass_type", &self.superclass_type())
            .field("num_immediate_members", &self.num_immediate_members())
            .field("num_fields", &self.num_fields())
            .field(
                "field_offset_vector_offset",
                &self.field_offset_vector_offset(),
            )
            .finish()
    }
}

impl ClassDescriptor {
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
    pub const unsafe fn from_raw(raw: RawClassDescriptor) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw type context descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawClassDescriptor {
        self.raw
    }
}

impl ClassDescriptor {
    /// Returns the type of the superclass, expressed as a mangled type name
    /// that can refer to the generic arguments of the subclass type.
    #[inline]
    pub fn superclass_type(&self) -> Option<&Mangled> {
        unsafe { self.superclass_type_ptr().as_ref() }
    }

    /// Returns a pointer to the mangled type name of the superclass.
    #[inline]
    pub fn superclass_type_ptr(&self) -> &RelativeDirectPointer<Mangled> {
        self.raw.superclass_type.cast_by_ref()
    }

    // TODO: Figure out how union fields should be exposed.

    /// Returns the number of additional members added by this class to the
    /// class metadata. This data is opaque by default to the runtime, other
    /// than as exposed in other members; it's really just
    /// `NumImmediateMembers * sizeof(void*)` bytes of data.
    ///
    /// Whether those bytes are added before or after the address point
    /// depends on `areImmediateMembersNegative()`.
    #[inline]
    pub fn num_immediate_members(&self) -> u32 {
        self.raw.num_immediate_members
    }

    /// The number of stored properties in the class, not including its
    /// superclasses. If there is a field offset vector, this is its length.
    #[inline]
    pub fn num_fields(&self) -> u32 {
        self.raw.num_fields
    }

    /// The offset of the field offset vector for this class's stored properties
    /// in its metadata, in words. 0 means there is no field offset vector.
    ///
    /// If this class has a resilient superclass, this offset is relative to the
    /// size of the resilient superclass metadata. Otherwise, it is absolute.
    #[inline]
    pub fn field_offset_vector_offset(&self) -> u32 {
        self.raw.field_offset_vector_offset
    }
}
