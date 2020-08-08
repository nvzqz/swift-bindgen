use crate::metadata::{ContextDescriptor, ContextDescriptorFlags};
use std::{fmt, ops::Deref, os::raw::c_char};
use swift_sys::{
    metadata::ModuleContextDescriptor as RawModuleContextDescriptor,
    ptr::{RelativeDirectPointerNonNull, RelativeIndirectablePointer},
};

/// A context descriptor for a module.
#[repr(transparent)]
pub struct ModuleContextDescriptor {
    raw: RawModuleContextDescriptor,
}

impl Deref for ModuleContextDescriptor {
    type Target = ContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<ContextDescriptor> for ModuleContextDescriptor {
    #[inline]
    fn as_ref(&self) -> &ContextDescriptor {
        self
    }
}

unsafe impl Send for ModuleContextDescriptor {}
unsafe impl Sync for ModuleContextDescriptor {}

impl fmt::Debug for ModuleContextDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format name field first to make nested output easier to follow.
        f.debug_struct("ModuleContextDescriptor")
            .field("name", &self.name())
            .field("flags", &self.flags())
            .field("parent", &self.parent())
            .finish()
    }
}

impl ModuleContextDescriptor {
    /// Creates an instance from a raw module context descriptor value.
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
    pub const unsafe fn from_raw(raw: RawModuleContextDescriptor) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw module context descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawModuleContextDescriptor {
        self.raw
    }
}

impl ModuleContextDescriptor {
    /// Creates a new module context descriptor.
    ///
    /// # Safety
    ///
    /// - The descriptor must have a memory layout appropriate for a module.
    ///
    /// - `flags` must indicate that this is a module.
    ///
    /// - `parent` must point to a valid descriptor that can represent a parent
    ///   module, or be null.
    ///
    /// - `name` must point to a valid UTF-8 C string.
    #[inline]
    pub const unsafe fn new(
        flags: ContextDescriptorFlags,
        parent: RelativeIndirectablePointer<ModuleContextDescriptor>,
        name: RelativeDirectPointerNonNull<c_char>,
    ) -> Self {
        Self {
            raw: RawModuleContextDescriptor {
                base: ContextDescriptor::new(flags, parent.cast()).into_raw(),
                name,
            },
        }
    }

    /// Returns this module's name.
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { self.name_ptr().as_str() }
    }

    /// Returns the relative direct pointer to this module's name.
    #[inline]
    pub fn name_ptr(&self) -> &RelativeDirectPointerNonNull<c_char> {
        &self.raw.name
    }

    /// Returns the parent context, or `None` if this is a top-level context.
    #[inline]
    pub fn parent(&self) -> Option<&ModuleContextDescriptor> {
        unsafe { self.parent_ptr().as_ref() }
    }

    /// Returns a relative pointer to the parent context.
    #[inline]
    pub fn parent_ptr(&self) -> &RelativeIndirectablePointer<ModuleContextDescriptor> {
        ContextDescriptor::parent_ptr(self).cast_by_ref()
    }
}

/// Checking against specific modules based on name.
impl ModuleContextDescriptor {
    // These checks are optimized for performance. They should all unroll to
    // individual byte checks.

    #[inline]
    unsafe fn name_eq(&self, cmp: &str) -> bool {
        // SAFETY: If `cmp` contains a null byte, then the loop may cause a
        // buffer overflow.
        debug_assert!(!cmp.as_bytes().contains(&0), "Null byte found in {:?}", cmp);

        let mut name = self.name_ptr().as_ptr().cast::<u8>();

        for &byte in cmp.as_bytes() {
            if *name != byte {
                return false;
            }

            name = name.add(1);
        }

        *name == b'0'
    }

    /// Returns `true` if this is the `Swift` module.
    pub fn is_swift(&self) -> bool {
        unsafe { self.name_eq("Swift") }
    }

    /// Returns `true` if this is the `SwiftUI` module.
    pub fn is_swift_ui(&self) -> bool {
        unsafe { self.name_eq("SwiftUI") }
    }

    /// Returns `true` if this is the `Combine` module.
    pub fn is_combine(&self) -> bool {
        unsafe { self.name_eq("Combine") }
    }

    /// Returns `true` if this is the `Builtin` module.
    pub fn is_builtin(&self) -> bool {
        unsafe { self.name_eq("Builtin") }
    }

    /// Returns `true` if this is a special C import module.
    pub fn is_c_imported(&self) -> bool {
        unsafe { self.name_eq("__C") }
    }
}
