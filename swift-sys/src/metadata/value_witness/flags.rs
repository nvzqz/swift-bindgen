use std::fmt;

/// Flags stored in the value-witness table.
///
/// Equivalent to `TargetValueWitnessFlags` in
/// [`MetadataValues.h`](https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataValues.h).
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueWitnessFlags {
    data: u32,
}

#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
enum Bits {
    AlignmentMask =       0x000000FF,
    // unused             0x0000FF00,
    IsNonPOD =            0x00010000,
    IsNonInline =         0x00020000,
    // unused             0x00040000,
    HasSpareBits =        0x00080000,
    IsNonBitwiseTakable = 0x00100000,
    HasEnumWitnesses =    0x00200000,
    Incomplete =          0x00400000,
    // unused             0xFF800000,
}

const MASK_KNOWN: u32 = Bits::AlignmentMask as u32
    | Bits::IsNonPOD as u32
    | Bits::IsNonInline as u32
    | Bits::HasSpareBits as u32
    | Bits::IsNonBitwiseTakable as u32
    | Bits::HasEnumWitnesses as u32
    | Bits::Incomplete as u32;

impl fmt::Debug for ValueWitnessFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ValueWitnessFlags");

        debug_struct
            .field("align", &self.align())
            .field("is_pod", &self.is_pod())
            .field("is_inline_storage", &self.is_inline_storage())
            .field("has_spare_bits", &self.is_inline_storage())
            .field("is_bitwise_takable", &self.is_bitwise_takable())
            .field("has_enum_witnesses", &self.has_enum_witnesses())
            .field("is_incomplete", &self.is_incomplete());

        // Format any unknown flags as bits with the known bits zeroed out.
        let unknown = self.data & !MASK_KNOWN;
        if unknown != 0 {
            debug_struct.field("unknown", &format_args!("{:#b}", unknown));
        }

        debug_struct.finish()
    }
}

impl ValueWitnessFlags {
    /// Creates flags from a 32-bit integer.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self { data: bits }
    }

    /// Returns the bits of the flags as a 32-bit integer.
    #[inline]
    pub const fn into_bits(self) -> u32 {
        self.data
    }

    /// Returns the required alignment of the first byte of an object of this
    /// type, expressed as a mask of the low bits that must not be set in the
    /// pointer.
    ///
    /// This representation can be easily converted to the 'alignof' result by
    /// merely adding 1, but it is more directly useful for performing dynamic
    /// structure layouts, and it grants an additional bit of precision in a
    /// compact field without needing to switch to an exponent representation.
    /// For example, if the type needs to be 8-byte aligned, the appropriate
    /// alignment mask should be `0x7`.
    #[inline]
    pub const fn align_mask(self) -> usize {
        (self.data & Bits::AlignmentMask as u32) as usize
    }

    /// Returns the required align of the first byte of an object of this
    /// type.
    #[inline]
    pub const fn align(self) -> usize {
        self.align_mask() + 1
    }

    /// Returns `true` if values of this type can be copied with `memcpy` and
    /// destroyed with a no-op.
    #[inline]
    pub const fn is_pod(self) -> bool {
        (self.data & Bits::IsNonPOD as u32) == 0
    }

    /// Returns `true` if the type requires out-of-line allocation of its
    /// storage.
    ///
    /// This can be the case because the value requires more storage or if it is
    /// not bitwise takable.
    #[inline]
    pub const fn is_inline_storage(self) -> bool {
        (self.data & Bits::IsNonInline as u32) == 0
    }

    /// Returns `true` if the type's binary representation has unused bits.
    #[inline]
    pub const fn has_spare_bits(self) -> bool {
        (self.data & Bits::HasSpareBits as u32) != 0
    }

    /// Returns `true` if values of this type can be taken with memcpy.
    ///
    /// Unlike C++ 'move', 'take' is a destructive operation that invalidates
    /// the source object, so most types can be taken with a simple bitwise
    /// copy. Only types with side table references, like `@weak` references, or
    /// types with opaque value semantics, like imported C++ types, are not
    /// bitwise-takable.
    #[inline]
    pub const fn is_bitwise_takable(self) -> bool {
        (self.data & Bits::IsNonBitwiseTakable as u32) == 0
    }

    /// Returns `true` if this type's binary representation is that of an enum,
    /// and the enum value witness table entries are available in this type's
    /// value witness table.
    #[inline]
    pub const fn has_enum_witnesses(self) -> bool {
        (self.data & Bits::HasEnumWitnesses as u32) != 0
    }

    /// Returns `true` if the type with this value-witness table is incomplete,
    /// meaning that its external layout (size, etc.) is meaningless pending
    /// completion of the metadata layout.
    #[inline]
    pub const fn is_incomplete(self) -> bool {
        (self.data & Bits::Incomplete as u32) != 0
    }
}
