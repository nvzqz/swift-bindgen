use crate::ctx_desc::{
    ContextDescriptorFlags, ContextDescriptorKind, ExtensionContextDescriptor,
    ModuleContextDescriptor, ProtocolContextDescriptor, TypeContextDescriptor,
};
use std::{fmt, hint, ptr};
use swift_sys::{
    ctx_desc::ContextDescriptor as RawContextDescriptor, ptr::RelativeIndirectablePointer,
};

/// Base class for all context descriptors.
#[repr(transparent)]
pub struct ContextDescriptor {
    raw: RawContextDescriptor,
}

impl AsRef<ContextDescriptor> for ContextDescriptor {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl Send for ContextDescriptor {}
unsafe impl Sync for ContextDescriptor {}

impl fmt::Debug for ContextDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format as the specific context descriptor type.
        //
        // `fmt` is called with the type's name to ensure that the correct
        // implementation calls, and that this does not infinitely recurse.
        match self.kind() {
            ContextDescriptorKind::MODULE => ModuleContextDescriptor::fmt(
                unsafe { &*(self as *const Self as *const ModuleContextDescriptor) },
                f,
            ),

            ContextDescriptorKind::EXTENSION => ExtensionContextDescriptor::fmt(
                unsafe { &*(self as *const Self as *const ExtensionContextDescriptor) },
                f,
            ),

            ContextDescriptorKind::PROTOCOL => ProtocolContextDescriptor::fmt(
                unsafe { &*(self as *const Self as *const ProtocolContextDescriptor) },
                f,
            ),

            // This case also handles classes.
            kind if kind.is_type() => TypeContextDescriptor::fmt(
                unsafe { &*(self as *const Self as *const TypeContextDescriptor) },
                f,
            ),

            // Default to "unknown" descriptor.
            _ => f
                .debug_struct("UnknownContextDescriptor")
                .field("flags", &self.flags())
                .field("parent", &self.parent())
                .finish(),
        }
    }
}

impl ContextDescriptor {
    /// Creates an instance from a raw context descriptor value.
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
    pub const unsafe fn from_raw(raw: RawContextDescriptor) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw context descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawContextDescriptor {
        self.raw
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
        Self {
            raw: RawContextDescriptor {
                flags,
                parent: parent.cast(),
            },
        }
    }

    /// Returns flags describing this context.
    #[inline]
    pub fn flags(&self) -> ContextDescriptorFlags {
        self.raw.flags
    }

    /// Returns the kind of this context descriptor.
    #[inline]
    pub fn kind(&self) -> ContextDescriptorKind {
        self.raw.flags.kind()
    }

    /// Returns the parent context, or `None` if this is a top-level context.
    #[inline]
    pub fn parent(&self) -> Option<&ContextDescriptor> {
        unsafe { self.parent_ptr().as_ref() }
    }

    /// Returns a relative pointer to the parent context.
    #[inline]
    pub fn parent_ptr(&self) -> &RelativeIndirectablePointer<ContextDescriptor> {
        self.raw.parent.cast_by_ref()
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

    /// Casts this context descriptor to an extension descriptor if it is one.
    #[inline]
    pub fn as_extension(&self) -> Option<&ExtensionContextDescriptor> {
        if self.kind() == ContextDescriptorKind::EXTENSION {
            Some(unsafe { &*(self as *const _ as *const _) })
        } else {
            None
        }
    }

    /// Casts this context descriptor to a nominal type descriptor if it is one.
    #[inline]
    pub fn as_type(&self) -> Option<&TypeContextDescriptor> {
        if self.kind().is_type() {
            Some(unsafe { &*(self as *const _ as *const _) })
        } else {
            None
        }
    }
}
