use crate::{
    metadata::{ContextDescriptor, ContextDescriptorFlags},
    ptr::{RelativeDirectPointerNonNull, RelativeIndirectablePointer},
};
use std::{fmt, ops::Deref, os::raw::c_char};

/// A context descriptor for a module.
#[repr(C)]
pub struct ModuleContextDescriptor {
    base: ContextDescriptor,
    name: RelativeDirectPointerNonNull<c_char>,
}

impl Deref for ModuleContextDescriptor {
    type Target = ContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
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
        f.debug_struct("ModuleContextDescriptor")
            .field("flags", &self.flags())
            .field("parent", &self.parent())
            .field("name", &self.name())
            .finish()
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
    ///   of the created descriptor.
    ///
    /// - `name` must point to a valid UTF-8 C string.
    #[inline]
    pub const unsafe fn new(
        flags: ContextDescriptorFlags,
        parent: RelativeIndirectablePointer<ModuleContextDescriptor>,
        name: RelativeDirectPointerNonNull<c_char>,
    ) -> Self {
        Self {
            base: ContextDescriptor::new(flags, parent.cast()),
            name,
        }
    }

    /// Returns the parent context, or `None` if this is a top-level context.
    #[inline]
    pub fn parent(&self) -> Option<&ModuleContextDescriptor> {
        unsafe { self.parent_ptr().as_ref() }
    }

    /// Returns a relative pointer to the parent context.
    #[inline]
    pub fn parent_ptr(&self) -> &RelativeIndirectablePointer<ModuleContextDescriptor> {
        self.base.parent_ptr().cast_by_ref()
    }

    /// Returns this module's name.
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { self.name.as_str() }
    }

    /// Returns the relative direct pointer to this module's name.
    #[inline]
    pub fn name_ptr(&self) -> &RelativeDirectPointerNonNull<c_char> {
        &self.name
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
