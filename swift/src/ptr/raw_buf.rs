use crate::{util::Nil, Int};
use std::{fmt, ops::Range, ptr};

/// A nonowning collection interface to the bytes in a region of memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsaferawbufferpointer).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnsafeRawBufferPointer {
    // SAFETY: This is equivalent in memory representation to
    // `UnsafeRawPointer?` in Swift.
    start: *const u8,
    end: *const u8,
}

impl From<&[u8]> for UnsafeRawBufferPointer {
    #[inline]
    fn from(bytes: &[u8]) -> Self {
        let Range { start, end } = bytes.as_ptr_range();
        Self { start, end }
    }
}

impl From<&mut [u8]> for UnsafeRawBufferPointer {
    #[inline]
    fn from(bytes: &mut [u8]) -> Self {
        (bytes as &[u8]).into()
    }
}

impl fmt::Debug for UnsafeRawBufferPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeRawBufferPointer")
            .field("start", Nil::fmt_debug_ptr(&self.start))
            .field("count", &self.count())
            .finish()
    }
}

impl fmt::Pointer for UnsafeRawBufferPointer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice_ptr().fmt(f)
    }
}

// TODO: Create initializers.
impl UnsafeRawBufferPointer {
    /// Returns the number of bytes in the buffer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/unsaferawbufferpointer/3019226-count).
    #[inline]
    pub fn count(&self) -> Int {
        if self.start.is_null() {
            0
        } else {
            (self.end as Int) - (self.start as Int)
        }
    }
}

impl UnsafeRawBufferPointer {
    /// Returns the number of bytes in the buffer as an unsigned integer.
    #[inline]
    pub fn len(&self) -> usize {
        // TODO: Implement `is_empty`.
        #![allow(clippy::len_without_is_empty)]

        self.count() as usize
    }

    /// Returns a raw pointer to the start of the buffer.
    #[inline]
    pub fn start(&self) -> *const u8 {
        self.start
    }

    /// Returns a raw pointer to the end of the buffer.
    #[inline]
    pub fn end(&self) -> *const u8 {
        self.end
    }

    /// Returns the two raw pointers spanning the buffer.
    #[inline]
    pub fn as_ptr_range(self) -> Range<*const u8> {
        self.start()..self.end()
    }

    /// Converts the buffer pointer to a byte slice pointer.
    #[inline]
    pub fn as_slice_ptr(self) -> *const [u8] {
        ptr::slice_from_raw_parts(self.start.cast::<u8>(), self.len())
    }

    // TODO: `as_slice`
}

/// A mutable nonowning collection interface to the bytes in a region of memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablerawbufferpointer).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnsafeMutableRawBufferPointer {
    // SAFETY: This is equivalent in memory representation to
    // `UnsafeMutableRawPointer?` in Swift.
    start: *mut u8,
    end: *mut u8,
}

impl From<UnsafeMutableRawBufferPointer> for UnsafeRawBufferPointer {
    #[inline]
    fn from(buf: UnsafeMutableRawBufferPointer) -> Self {
        Self {
            start: buf.start,
            end: buf.end,
        }
    }
}

impl From<UnsafeRawBufferPointer> for UnsafeMutableRawBufferPointer {
    #[inline]
    fn from(buf: UnsafeRawBufferPointer) -> Self {
        Self {
            start: buf.start as *mut u8,
            end: buf.end as *mut u8,
        }
    }
}

impl From<&mut [u8]> for UnsafeMutableRawBufferPointer {
    #[inline]
    fn from(bytes: &mut [u8]) -> Self {
        let Range { start, end } = bytes.as_mut_ptr_range();
        Self { start, end }
    }
}

impl fmt::Debug for UnsafeMutableRawBufferPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeMutableRawBufferPointer")
            .field("start", Nil::fmt_debug_mut_ptr(&self.start))
            .field("count", &self.count())
            .finish()
    }
}

impl fmt::Pointer for UnsafeMutableRawBufferPointer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.start.fmt(f)
    }
}

// TODO: Create initializers.
impl UnsafeMutableRawBufferPointer {
    /// Returns the number of bytes in the buffer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablerawbufferpointer/3019191-count).
    #[inline]
    pub fn count(&self) -> Int {
        if self.start.is_null() {
            0
        } else {
            (self.end as Int) - (self.start as Int)
        }
    }
}

impl UnsafeMutableRawBufferPointer {
    /// Returns the number of bytes in the buffer as an unsigned integer.
    #[inline]
    pub fn len(&self) -> usize {
        // TODO: Implement `is_empty`.
        #![allow(clippy::len_without_is_empty)]

        self.count() as usize
    }

    /// Returns a raw pointer to the start of the buffer.
    #[inline]
    pub fn start(&self) -> *mut u8 {
        self.start
    }

    /// Returns a raw pointer to the end of the buffer.
    #[inline]
    pub fn end(&self) -> *mut u8 {
        self.end
    }

    /// Returns the two raw pointers spanning the buffer.
    #[inline]
    pub fn as_ptr_range(self) -> Range<*mut u8> {
        self.start()..self.end()
    }

    /// Converts the buffer pointer to a byte slice pointer.
    #[inline]
    pub fn as_slice_ptr(self) -> *mut [u8] {
        ptr::slice_from_raw_parts_mut(self.start.cast::<u8>(), self.len())
    }

    // TODO: `as_slice` and `as_slice_mut`
}
