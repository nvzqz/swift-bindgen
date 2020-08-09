use crate::ctx_desc::ContextDescriptor;
use std::{fmt, ops::Deref, os::raw::c_char};
use swift_sys::{
    ctx_desc::ProtocolContextDescriptor as RawProtocolContextDescriptor,
    ptr::{
        RelativeDirectPointer, RelativeDirectPointerNonNull, RelativeIndirectablePointerNonNull,
    },
};

/// Context descriptor for a protocol.
#[repr(transparent)]
pub struct ProtocolContextDescriptor {
    raw: RawProtocolContextDescriptor,
}

impl Deref for ProtocolContextDescriptor {
    type Target = ContextDescriptor;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl AsRef<ContextDescriptor> for ProtocolContextDescriptor {
    #[inline]
    fn as_ref(&self) -> &ContextDescriptor {
        self
    }
}

unsafe impl Send for ProtocolContextDescriptor {}
unsafe impl Sync for ProtocolContextDescriptor {}

impl fmt::Debug for ProtocolContextDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct AssociatedTypeNames<'a>(&'a str);

        impl fmt::Debug for AssociatedTypeNames<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Associated type names are space-separated.
                f.debug_list()
                    .entries(self.0.split(' ').filter(|name| !name.is_empty()))
                    .finish()
            }
        }

        // Always format associated type names as list.
        let associated_type_names: AssociatedTypeNames = match self.associated_type_names_str() {
            Some(names) => AssociatedTypeNames(names),
            None => AssociatedTypeNames(Default::default()),
        };

        // Format name and associated type fields before other fields to make
        // output easier to follow.
        f.debug_struct("ProtocolContextDescriptor")
            .field("name", &self.name())
            .field("associated_type_names", &associated_type_names)
            .field(
                // TODO: Format protocol-specific flags as part of this
                // property.
                "flags",
                &self.flags(),
            )
            .field("parent", &self.parent())
            .field(
                "num_requirements_in_signature",
                &self.num_requirements_in_signature(),
            )
            .field("num_requirements", &self.num_requirements())
            .finish()
    }
}

impl ProtocolContextDescriptor {
    /// Creates an instance from a raw protocol context descriptor value.
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
    pub const unsafe fn from_raw(raw: RawProtocolContextDescriptor) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw protocol context descriptor value.
    #[inline]
    pub const fn into_raw(self) -> RawProtocolContextDescriptor {
        self.raw
    }
}

impl ProtocolContextDescriptor {
    /// Returns the name of the protocol.
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { self.name_ptr().as_str() }
    }

    /// Returns a pointer to the name of the protocol.
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
        // SAFETY: Protocols are never a top-level context, so they always have
        // a parent context.
        unsafe { ContextDescriptor::parent_ptr(self).as_non_null() }
    }

    // TODO: Create helper type for enumerating space-separated type names.
    //
    // pub fn associated_type_names(&self) -> SpaceSeparatedList

    /// Returns a string containing space-separated names of associated types,
    /// or `None` if the protocol has no associated types.
    #[inline]
    pub fn associated_type_names_str(&self) -> Option<&str> {
        unsafe { self.associated_type_names_ptr().as_str() }
    }

    /// Returns a C string pointer containing space-separated names of
    /// associated types, or null if the protocol has no associated types.
    #[inline]
    pub fn associated_type_names_ptr(&self) -> &RelativeDirectPointer<c_char> {
        &self.raw.associated_type_names
    }

    /// Returns the number of generic requirements in the requirement signature of the
    /// protocol.
    #[inline]
    pub fn num_requirements_in_signature(&self) -> u32 {
        self.raw.num_requirements_in_signature
    }

    /// Returns the number of requirements in the protocol.
    ///
    /// If any requirements beyond `MinimumWitnessTableSizeInWords` are present
    /// in the witness table template, they will be not be overwritten with
    /// defaults.
    #[inline]
    pub fn num_requirements(&self) -> u32 {
        self.raw.num_requirements
    }

    // TODO: Create methods for trailing generic requirements and protocol
    // requirements.
}
