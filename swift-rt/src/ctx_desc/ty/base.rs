use crate::{
    ctx_desc::{ContextDescriptor, ContextDescriptorFlags},
    reflection::FieldDescriptor,
};
use std::{fmt, ops::Deref, os::raw::c_char};
use swift_sys::{
    ctx_desc::TypeContextDescriptor as RawTypeContextDescriptor,
    metadata::MetadataAccessFunction,
    ptr::{
        RelativeDirectPointer, RelativeDirectPointerNonNull, RelativeIndirectablePointerNonNull,
    },
};

/// Context descriptor for any nominal type.
#[repr(transparent)]
pub struct TypeContextDescriptor {
    raw: RawTypeContextDescriptor,
}

impl Deref for TypeContextDescriptor {
    type Target = ContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<ContextDescriptor> for TypeContextDescriptor {
    #[inline]
    fn as_ref(&self) -> &ContextDescriptor {
        self
    }
}

unsafe impl Send for TypeContextDescriptor {}
unsafe impl Sync for TypeContextDescriptor {}

impl fmt::Debug for TypeContextDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Dynamically format as the appropriate subtype.

        // Format name field first to make nested output easier to follow.
        f.debug_struct("TypeContextDescriptor")
            .field("name", &self.name())
            .field("flags", &self.flags())
            .field("parent", &self.parent())
            .field("access_function", &self.access_function())
            .field("fields", &self.fields())
            .finish()
    }
}

impl TypeContextDescriptor {
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
    pub const unsafe fn from_raw(raw: RawTypeContextDescriptor) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw type context descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawTypeContextDescriptor {
        self.raw
    }
}

impl TypeContextDescriptor {
    /// Creates a new nominal type context descriptor.
    ///
    /// # Safety
    ///
    /// - The descriptor must have a memory layout appropriate for the kind of
    ///   type specified by `flags`.
    ///
    /// - `flags` must indicate that this is a type.
    ///
    /// - `parent` must point to a valid descriptor that can represent a parent
    ///   module.
    ///
    /// - `name` must point to a valid UTF-8 C string.
    ///
    /// - `access_function` must point to a valid metadata access function for
    ///   the type, or be null.
    ///
    /// - `fields` must point to a valid field descriptor for the type, or be
    ///   null.
    #[inline]
    pub const unsafe fn new(
        flags: ContextDescriptorFlags,
        parent: RelativeIndirectablePointerNonNull<ContextDescriptor>,
        name: RelativeDirectPointerNonNull<c_char>,
        access_function: RelativeDirectPointer<MetadataAccessFunction>,
        fields: RelativeDirectPointer<FieldDescriptor>,
    ) -> Self {
        Self {
            raw: RawTypeContextDescriptor {
                base: ContextDescriptor::new(flags, parent.into_nullable().cast()).into_raw(),
                name,
                access_function,
                fields: fields.cast(),
            },
        }
    }

    /// Returns the name of the type.
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { self.name_ptr().as_str() }
    }

    /// Returns a pointer to the name of the type.
    #[inline]
    pub fn name_ptr(&self) -> &RelativeDirectPointerNonNull<c_char> {
        &self.raw.name
    }

    /// Returns the parent context.
    #[inline]
    pub fn parent(&self) -> &ContextDescriptor {
        unsafe { self.parent_ptr().as_ref() }
    }

    /// Returns a relative pointer to the parent context.
    #[inline]
    pub fn parent_ptr(&self) -> &RelativeIndirectablePointerNonNull<ContextDescriptor> {
        // SAFETY: Types are never a top-level context, so they always have a
        // parent context.
        unsafe { ContextDescriptor::parent_ptr(self).as_non_null() }
    }

    /// Returns the metadata access function for the type.
    #[inline]
    pub fn access_function(&self) -> Option<MetadataAccessFunction> {
        Some(unsafe { *self.access_function_ptr().as_ref()? })
    }

    /// Returns a pointer to the metadata access function for the type.
    #[inline]
    pub fn access_function_ptr(&self) -> &RelativeDirectPointer<MetadataAccessFunction> {
        &self.raw.access_function
    }

    /// Returns a reference to the field descriptor for the type, if any.
    #[inline]
    pub fn fields(&self) -> Option<&FieldDescriptor> {
        unsafe { self.fields_ptr().as_ref() }
    }

    /// Returns a pointer to the field descriptor for the type, if any.
    #[inline]
    pub fn fields_ptr(&self) -> &RelativeDirectPointer<FieldDescriptor> {
        self.raw.fields.cast_by_ref()
    }

    /// Returns `true` if the fields of the type can be inspected through
    /// reflection.
    #[inline]
    pub fn is_reflectable(&self) -> bool {
        !self.fields_ptr().is_null()
    }
}
