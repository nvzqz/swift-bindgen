use std::{
    ascii,
    fmt::{self, Write},
    mem,
    os::raw::c_void,
    slice,
};

/// A mangled Swift symbol.
#[repr(C)]
pub struct Mangled {
    data: [u8; 0],
}

impl fmt::Debug for Mangled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"")?;

        for &byte in self.to_bytes() {
            for escaped in ascii::escape_default(byte) {
                f.write_char(escaped as char)?;
            }
        }

        write!(f, "\"")
    }
}

impl Mangled {
    /// Returns the offset to the next normal byte in a symbol.
    #[inline]
    fn offset_of(byte: u8) -> usize {
        // See `makeSymbolicMangledNameStringRef` implementation.
        match byte {
            0 => 0,

            // Skip over symbolic references.
            0x01..=0x17 => 1 + mem::size_of::<u32>(),
            0x18..=0x1F => 1 + mem::size_of::<*const c_void>(),

            _ => 1,
        }
    }

    /// Returns the number of bytes in the symbol string.
    pub fn len(&self) -> usize {
        let start = self.as_ptr();
        let mut len = 0;

        loop {
            let offset = Self::offset_of(unsafe { *start.add(len) });
            if offset == 0 {
                return len;
            }

            len += offset;
        }
    }

    /// Returns a pointer to the start of the symbol string.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    /// Converts this symbol string to a byte slice.
    ///
    /// A length calculation is performed whenever this method is called.
    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    /// Converts this symbol string to a byte slice containing the trailing 0
    /// byte.
    ///
    /// A length calculation is performed whenever this method is called.
    #[inline]
    pub fn to_bytes_with_nul(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len() + 1) }
    }
}
