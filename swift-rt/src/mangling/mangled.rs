use std::{
    ascii,
    fmt::{self, Write},
    mem,
    os::raw::c_void,
    slice, str,
};

/// A mangled Swift symbol.
///
/// # Debug formatting
///
/// The `Debug` implementation of this type takes into account relative and
/// absolute references (delimited by `0x01..=0x17` and `0x18..=0x1F`
/// respectively) encoded in the symbol.
///
/// References are formatted as `<` + `$delim` (hex) + `:` + `$offset` or
/// `$address` + `>`.
#[repr(C)]
pub struct Mangled {
    data: [u8; 0],
}

impl fmt::Debug for Mangled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"")?;

        let mut current = self.as_ptr();

        loop {
            // SAFETY: `current` always refers to a byte within bounds.
            let next = unsafe { *current };

            match Component::new(next) {
                Component::Null => break,
                Component::SymbolicReference(reference) => {
                    fn hexify(b: u8) -> u8 {
                        match b {
                            0..=9 => b'0' + b,
                            _ => b'a' + b - 10,
                        }
                    }

                    // SAFETY: The string in the form "<XX:" is UTF-8.
                    let prefix = [b'<', hexify(next >> 4), hexify(next & 0xf), b':'];
                    let prefix: &str = unsafe { str::from_utf8_unchecked(&prefix) };
                    f.write_str(prefix)?;

                    match reference {
                        // SAFETY: A relative symbolic reference delimiter
                        // ensures that the value following the delimiter is a
                        // 32-bit integer for the offset.
                        SymbolicReference::Relative => unsafe {
                            let offset_ptr = current.add(1).cast::<i32>();
                            current = offset_ptr.add(1).cast();

                            let offset = offset_ptr.read_unaligned();
                            write!(f, "{:?}", offset)?;
                        },

                        // SAFETY: An absolute symbolic reference delimiter
                        // ensures that the value following the delimiter is a
                        // pointer.
                        SymbolicReference::Absolute => unsafe {
                            let addr_ptr = current.add(1).cast::<*const c_void>();
                            current = addr_ptr.add(1).cast();

                            let addr = addr_ptr.read_unaligned();
                            write!(f, "{:?}", addr)?;
                        },
                    }

                    f.write_char('>')?;
                }
                Component::Normal => {
                    for escaped in ascii::escape_default(next) {
                        f.write_char(escaped as char)?;
                    }

                    // SAFETY: There is another byte to decode in the name.
                    current = unsafe { current.add(1) };
                }
            }
        }

        write!(f, "\"")
    }
}

#[derive(Clone, Copy)]
enum Component {
    Null,
    SymbolicReference(SymbolicReference),
    Normal,
}

#[derive(Clone, Copy)]
enum SymbolicReference {
    Relative,
    Absolute,
}

// See `makeSymbolicMangledNameStringRef` implementation.
impl Component {
    #[inline]
    pub fn new(byte: u8) -> Self {
        match byte {
            0 => Self::Null,
            0x01..=0x17 => Self::SymbolicReference(SymbolicReference::Relative),
            0x18..=0x1F => Self::SymbolicReference(SymbolicReference::Absolute),
            _ => Self::Normal,
        }
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
