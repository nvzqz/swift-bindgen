use crate::ctx_desc::ContextDescriptorKind;
use std::fmt;

/// Flags for a context descriptor.
///
/// Stored in the first 32-bit word of any context descriptor.
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextDescriptorFlags(u32);

impl fmt::Debug for ContextDescriptorFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ContextDescriptorFlags")
            .field("kind", &self.kind())
            .field("is_generic", &self.is_generic())
            .field("is_unique", &self.is_unique())
            .field("version", &self.version())
            .field(
                // Format flags as bits.
                "kind_specific_flags",
                &format_args!("{:#b}", self.kind_specific_flags()),
            )
            .finish()
    }
}

impl ContextDescriptorFlags {
    /// Creates a new instance without any bits set.
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Creates a new set of flags.
    #[inline]
    pub const fn new(
        kind: ContextDescriptorKind,
        is_generic: bool,
        is_unique: bool,
        version: u8,
        kind_specific_flags: u16,
    ) -> Self {
        Self::empty()
            .with_kind(kind)
            .with_generic(is_generic)
            .with_unique(is_unique)
            .with_version(version)
            .with_kind_specific_flags(kind_specific_flags)
    }

    /// Creates a new instance from `bits` without checking validity.
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> Self {
        // TODO: Add safety doc section.
        #![allow(clippy::missing_safety_doc)]

        Self(bits)
    }

    /// Returns the underlying bits of `self`.
    #[inline]
    pub const fn into_bits(self) -> u32 {
        self.0
    }

    /// Returns the kind of context this descriptor describes.
    #[inline]
    pub const fn kind(self) -> ContextDescriptorKind {
        unsafe { ContextDescriptorKind::new_unchecked((self.0 & 0x1F) as u8) }
    }

    /// Returns whether the context being described is generic.
    #[inline]
    pub const fn is_generic(self) -> bool {
        self.0 & 0x80 != 0
    }

    /// Returns whether this is a unique record describing the referenced
    /// context.
    #[inline]
    pub const fn is_unique(self) -> bool {
        self.0 & 0x40 != 0
    }

    /// Returns the format version of the descriptor. Higher version numbers may
    /// have additional fields that aren't present in older versions.
    #[inline]
    pub const fn version(self) -> u8 {
        (self.0 >> 8) as u8
    }

    /// Returns the most significant two bytes of the flags word, which can have
    /// kind-specific meaning.
    #[inline]
    pub const fn kind_specific_flags(self) -> u16 {
        (self.0 >> 16) as u16
    }

    /// Returns `self` with `kind` set.
    #[inline]
    pub const fn with_kind(self, kind: ContextDescriptorKind) -> Self {
        Self((self.into_bits() & 0xFFFFFFE0) | kind.value() as u32)
    }

    /// Returns `self` with `is_generic` set.
    #[inline]
    pub const fn with_generic(self, is_generic: bool) -> Self {
        Self((self.into_bits() & 0xFFFFFF7F) | (0x80 * is_generic as u32))
    }

    /// Returns `self` with `is_unique` set.
    #[inline]
    pub const fn with_unique(self, is_unique: bool) -> Self {
        Self((self.into_bits() & 0xFFFFFFBF) | (0x40 * is_unique as u32))
    }

    /// Returns `self` with `version` set.
    #[inline]
    pub const fn with_version(self, version: u8) -> Self {
        Self((self.into_bits() & 0xFFFF00FF) | ((version as u32) << 8))
    }

    /// Returns `self` with `kind_specific_flags` set.
    #[inline]
    pub const fn with_kind_specific_flags(self, flags: u16) -> Self {
        Self((self.into_bits() & 0xFFFF0000) | ((flags as u32) << 16))
    }
}
