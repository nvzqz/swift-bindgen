use crate::{
    mangling::Mangled,
    metadata::{
        ContextDescriptor, ContextDescriptorFlags, ContextDescriptorKind, ModuleContextDescriptor,
    },
};
use std::{fmt, ops::Deref};
use swift_sys::{
    metadata::ExtensionContextDescriptor as RawExtensionContextDescriptor,
    ptr::{RelativeDirectPointer, RelativeIndirectablePointerNonNull},
};

/// Descriptor for an extension context.
#[repr(transparent)]
pub struct ExtensionContextDescriptor {
    raw: RawExtensionContextDescriptor,
}

impl Deref for ExtensionContextDescriptor {
    type Target = ContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<ContextDescriptor> for ExtensionContextDescriptor {
    #[inline]
    fn as_ref(&self) -> &ContextDescriptor {
        self
    }
}

unsafe impl Send for ExtensionContextDescriptor {}
unsafe impl Sync for ExtensionContextDescriptor {}

impl fmt::Debug for ExtensionContextDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ExtensionContextDescriptor")
            .field("flags", &self.flags())
            .field("parent", self.parent())
            .field("extended_context", &self.extended_context())
            .finish()
    }
}

impl ExtensionContextDescriptor {
    /// Creates a new extension context descriptor.
    ///
    /// # Safety
    ///
    /// - The descriptor must have a memory layout appropriate for an extension.
    ///
    /// - `flags` must indicate that this is an extension.
    ///
    /// - `parent` must point to a valid module descriptor.
    ///
    /// - `extended_context` must point to a mangled name of the `Self` type
    ///   context.
    #[inline]
    pub const unsafe fn new(
        flags: ContextDescriptorFlags,
        parent: RelativeIndirectablePointerNonNull<ModuleContextDescriptor>,
        extended_context: RelativeDirectPointer<Mangled>,
    ) -> Self {
        Self {
            raw: RawExtensionContextDescriptor {
                base: ContextDescriptor::new(flags, parent.cast().into_nullable()).into_raw(),
                extended_context: extended_context.cast(),
            },
        }
    }

    /// Returns the parent module context.
    #[inline]
    pub fn parent(&self) -> &ModuleContextDescriptor {
        let parent = unsafe { self.parent_ptr().as_ref() };

        // From a comment on the `ExtendedContext` field in the original source:
        // > Note that the Parent of the extension will be the module context
        // > the extension is declared inside.
        //
        // We assert this in debug builds to catch this assumption being broken.
        debug_assert_eq!(parent.flags().kind(), ContextDescriptorKind::MODULE);

        parent
    }

    /// Returns a relative pointer to the parent context.
    #[inline]
    pub fn parent_ptr(&self) -> &RelativeIndirectablePointerNonNull<ModuleContextDescriptor> {
        // SAFETY: Extensions are never a top-level context, so they always have
        // a parent context.
        unsafe { self.raw.base.parent.as_non_null().cast_by_ref() }
    }

    /// Returns a mangling of the `Self` type context that the extension
    /// extends.
    ///
    /// The mangled name represents the type in the generic context encoded by
    /// this descriptor. For example, a nongeneric nominal type extension will
    /// encode the nominal type name. A generic nominal type extension will
    /// encode the instance of the type with any generic arguments bound.
    #[inline]
    pub fn extended_context(&self) -> Option<&Mangled> {
        unsafe { self.extended_context_ptr().as_ref() }
    }

    /// Returns a pointer to the mangling of the `Self` type context that the
    /// extension extends.
    #[inline]
    pub fn extended_context_ptr(&self) -> &RelativeDirectPointer<Mangled> {
        self.raw.extended_context.cast_by_ref()
    }
}
