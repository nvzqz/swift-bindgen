use crate::ctx_desc::TypeContextDescriptor;

/// Context descriptor for an enum type.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct EnumDescriptor {
    /// The base nominal type descriptor.
    pub base: TypeContextDescriptor,

    /// The number of non-empty cases in the enum are in the low 24 bits; the
    /// offset of the payload size in the metadata record in words, if any, is
    /// stored in the high 8 bits.
    pub num_payload_cases_and_payload_size_offset: u32,

    /// The number of empty cases in the enum.
    pub num_empty_cases: u32,
}

impl EnumDescriptor {
    /// Returns the number of non-empty cases in the enum.
    #[inline]
    pub const fn num_payload_cases(&self) -> u32 {
        self.num_payload_cases_and_payload_size_offset & 0x00FFFFFF
    }

    /// Returns the total number of cases in the enum.
    #[inline]
    pub const fn num_cases(&self) -> u32 {
        self.num_payload_cases() + self.num_empty_cases
    }

    /// Returns the offset of the payload size in the metadata record in words,
    /// if any.
    #[inline]
    pub const fn payload_size_offset(&self) -> u8 {
        ((self.num_payload_cases_and_payload_size_offset & 0xFF000000) >> 24) as u8
    }
}
