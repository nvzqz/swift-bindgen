use std::fmt;

/// Flags for interpreting semantics of a field record.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldRecordFlags {
    data: u32,
}

const FLAG_IS_INDIRECT: u32 = 0x1;
const FLAG_IS_VAR: u32 = 0x2;
const FLAG_IS_ARTIFICIAL: u32 = 0x4;

// A bit mask of all known flags at the time of this writing.
const MASK_KNOWN: u32 = FLAG_IS_INDIRECT | FLAG_IS_VAR | FLAG_IS_ARTIFICIAL;

impl fmt::Debug for FieldRecordFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_struct = f.debug_struct("FieldRecordFlags");

        debug_struct
            .field("is_indirect", &self.is_indirect())
            .field("is_var", &self.is_var())
            .field("is_artificial", &self.is_artificial());

        // Format any unknown flags as bits with the known bits zeroed out.
        let unknown = self.data & !MASK_KNOWN;
        if unknown != 0 {
            debug_struct.field("unknown", &format_args!("{:#b}", unknown));
        }

        debug_struct.finish()
    }
}

impl FieldRecordFlags {
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

    /// Returns `true` if the record refers to an indirect enum case.
    #[inline]
    pub const fn is_indirect(&self) -> bool {
        self.data & FLAG_IS_INDIRECT != 0
    }

    /// Returns `self` with `is_indirect` set.
    #[inline]
    pub const fn with_indirect(self, is_indirect: bool) -> Self {
        Self {
            data: (self.data & !FLAG_IS_INDIRECT) | (FLAG_IS_INDIRECT * is_indirect as u32),
        }
    }

    /// Returns `true` if the field is a mutable `var` property
    #[inline]
    pub const fn is_var(&self) -> bool {
        self.data & FLAG_IS_VAR != 0
    }

    /// Returns `self` with `is_var` set.
    #[inline]
    pub const fn with_var(self, is_var: bool) -> Self {
        Self {
            data: (self.data & !FLAG_IS_VAR) | (FLAG_IS_VAR * is_var as u32),
        }
    }

    /// Returns `true` if the record refers to an artificial field.
    #[inline]
    pub const fn is_artificial(&self) -> bool {
        self.data & FLAG_IS_ARTIFICIAL != 0
    }

    /// Returns `self` with `is_artificial` set.
    #[inline]
    pub const fn with_artificial(self, is_artificial: bool) -> Self {
        Self {
            data: (self.data & !FLAG_IS_ARTIFICIAL) | (FLAG_IS_ARTIFICIAL * is_artificial as u32),
        }
    }
}
