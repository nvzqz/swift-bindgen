use crate::{util::Nil, Int};
use std::{cmp::Ordering, fmt, hash, ptr};

/// A nonowning collection interface to a buffer of elements stored contiguously
/// in memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafebufferpointer).
#[repr(C)]
pub struct UnsafeBufferPointer<T> {
    // SAFETY: This is equivalent in memory representation to
    // `UnsafePointer<T>?` in Swift.
    start: *const T,

    count: Int,
}

impl<T> Clone for UnsafeBufferPointer<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UnsafeBufferPointer<T> {}

impl<T> fmt::Debug for UnsafeBufferPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeBufferPointer")
            .field("start", Nil::fmt_debug_ptr(&self.start))
            .field("count", &self.count)
            .finish()
    }
}

impl<T> fmt::Pointer for UnsafeBufferPointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.start.fmt(f)
    }
}

impl<T> PartialEq for UnsafeBufferPointer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.count == other.count
    }
}

impl<T> Eq for UnsafeBufferPointer<T> {}

impl<T> PartialOrd for UnsafeBufferPointer<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for UnsafeBufferPointer<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.count.cmp(&other.count),
            ordering => ordering,
        }
    }
}

impl<T> hash::Hash for UnsafeBufferPointer<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.count.hash(state);
    }
}

impl<T> UnsafeBufferPointer<T> {
    /// Returns the number of elements in the buffer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/unsafebufferpointer/2943791-count).
    #[inline]
    pub fn count(&self) -> Int {
        self.count
    }
}

impl<T> UnsafeBufferPointer<T> {
    /// Returns the number of elements in the buffer as an unsigned integer.
    #[inline]
    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Converts the buffer pointer to a slice pointer.
    #[inline]
    pub fn as_slice_ptr(self) -> *const [T] {
        ptr::slice_from_raw_parts(self.start, self.len())
    }

    // TODO: `as_slice`
}

/// A nonowning collection interface to a buffer of mutable elements stored
/// contiguously in memory.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablebufferpointer).
#[repr(C)]
pub struct UnsafeMutableBufferPointer<T> {
    // SAFETY: This is equivalent in memory representation to
    // `UnsafeMutablePointer<T>?` in Swift.
    start: *mut T,

    count: Int,
}

impl<T> Clone for UnsafeMutableBufferPointer<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UnsafeMutableBufferPointer<T> {}

impl<T> fmt::Debug for UnsafeMutableBufferPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnsafeMutableBufferPointer")
            .field("start", Nil::fmt_debug_mut_ptr(&self.start))
            .field("count", &self.count)
            .finish()
    }
}

impl<T> fmt::Pointer for UnsafeMutableBufferPointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.start.fmt(f)
    }
}

impl<T> PartialEq for UnsafeMutableBufferPointer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.count == other.count
    }
}

impl<T> Eq for UnsafeMutableBufferPointer<T> {}

impl<T> PartialOrd for UnsafeMutableBufferPointer<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for UnsafeMutableBufferPointer<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.count.cmp(&other.count),
            ordering => ordering,
        }
    }
}

impl<T> hash::Hash for UnsafeMutableBufferPointer<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.count.hash(state);
    }
}

impl<T> UnsafeMutableBufferPointer<T> {
    /// Returns the number of elements in the buffer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablebufferpointer/2943832-count).
    #[inline]
    pub fn count(&self) -> Int {
        self.count
    }
}

impl<T> UnsafeMutableBufferPointer<T> {
    /// Returns the number of elements in the buffer as an unsigned integer.
    #[inline]
    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Converts the buffer pointer to a slice pointer.
    #[inline]
    pub fn as_slice_ptr(self) -> *mut [T] {
        ptr::slice_from_raw_parts_mut(self.start, self.len())
    }

    // TODO: `as_slice` and `as_slice_mut`
}
