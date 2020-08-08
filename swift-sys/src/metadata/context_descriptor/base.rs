use crate::{
    metadata::{ContextDescriptorFlags, ContextDescriptorKind, ModuleContextDescriptor},
    ptr::RelativeIndirectablePointer,
};
use std::{hint, ptr};

/// Base class for all context descriptors.
#[repr(C)]
pub struct ContextDescriptor {
    /// Flags describing the context, including its kind and format version.
    flags: ContextDescriptorFlags,

    /// The parent context, or null if this is a top-level context.
    parent: RelativeIndirectablePointer<ContextDescriptor>,
}

unsafe impl Send for ContextDescriptor {}
unsafe impl Sync for ContextDescriptor {}

impl AsRef<ContextDescriptor> for ContextDescriptor {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ContextDescriptor {
    /// Creates a new context descriptor.
    ///
    /// # Safety
    ///
    /// - The descriptor must have a memory layout appropriate for the type of
    ///   descriptor indicated by `flags`. This often includes data that is
    ///   placed immediately after the created instance.
    ///
    /// - `parent` must point to a valid descriptor that can represent a parent
    ///   of the created descriptor. It must also be appropriate for the
    ///   descriptor kind.
    #[inline]
    pub const unsafe fn new(
        flags: ContextDescriptorFlags,
        parent: RelativeIndirectablePointer<ContextDescriptor>,
    ) -> Self {
        Self { flags, parent }
    }

    /// Returns flags describing this context.
    #[inline]
    pub fn flags(&self) -> ContextDescriptorFlags {
        self.flags
    }

    /// Returns the kind of this context descriptor.
    #[inline]
    pub fn kind(&self) -> ContextDescriptorKind {
        self.flags.kind()
    }

    /// Returns the parent context, or `None` if this is a top-level context.
    #[inline]
    pub fn parent(&self) -> Option<&ContextDescriptor> {
        unsafe { self.parent.as_ref() }
    }

    /// Returns a relative pointer to the parent context.
    #[inline]
    pub fn parent_ptr(&self) -> &RelativeIndirectablePointer<ContextDescriptor> {
        &self.parent
    }

    /// Returns an iterator over the parent contexts of `self`.
    #[inline]
    pub fn parent_iter(&self) -> impl Iterator<Item = &ContextDescriptor> + Copy {
        #[derive(Copy, Clone)]
        struct Iter<'a>(&'a ContextDescriptor);

        impl<'a> Iterator for Iter<'a> {
            type Item = &'a ContextDescriptor;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let parent = self.0.parent()?;
                self.0 = parent;
                Some(parent)
            }
        }

        // There are no more parents after the root is reached.
        impl std::iter::FusedIterator for Iter<'_> {}

        Iter(self)
    }

    /// Returns `true` if the given context descriptor is in the parent
    /// hierarchy of `self`.
    pub fn has_parent(&self, desc: &ContextDescriptor) -> bool {
        self.parent_iter().any(|parent| ptr::eq(parent, desc))
    }

    /// Returns the module context for `self`.
    #[inline]
    pub fn module_context(&self) -> &ModuleContextDescriptor {
        let mut current = self;
        loop {
            if let Some(module) = current.as_module() {
                return module;
            } else if let Some(parent) = current.parent() {
                current = parent;
            } else {
                // The runtime assumes that all context chains should eventually
                // find a module.
                unsafe { hint::unreachable_unchecked() };
            }
        }
    }
}

/// Casting to subtypes.
impl ContextDescriptor {
    /// Casts this context descriptor to a module descriptor if it is one.
    #[inline]
    pub fn as_module(&self) -> Option<&ModuleContextDescriptor> {
        if self.kind() == ContextDescriptorKind::MODULE {
            Some(unsafe { &*(self as *const _ as *const _) })
        } else {
            None
        }
    }
}
