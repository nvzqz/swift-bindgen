use std::{cmp::Ordering, fmt, hash, ptr::NonNull};

/// A pointer for accessing data of a specific type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafepointer).
#[repr(transparent)]
pub struct UnsafePointer<T> {
    inner: NonNull<T>,
}

impl<T> Clone for UnsafePointer<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UnsafePointer<T> {}

impl<T> fmt::Debug for UnsafePointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> fmt::Pointer for UnsafePointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> PartialEq for UnsafePointer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for UnsafePointer<T> {}

impl<T> PartialOrd for UnsafePointer<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for UnsafePointer<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> hash::Hash for UnsafePointer<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> UnsafePointer<T> {
    /// Creates a new instance from an immutable Rust pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *const T) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr as *mut T),
        }
    }

    /// Creates a new instance from an immutable Rust pointer if it is non-null.
    #[inline]
    pub fn new(ptr: *const T) -> Option<Self> {
        Some(Self {
            inner: NonNull::new(ptr as *mut T)?,
        })
    }

    /// Acquires the underlying immutable pointer.
    #[inline]
    pub const fn as_ptr(self) -> *const T {
        self.inner.as_ptr()
    }

    // TODO: `as_ref`
}

/// A pointer for accessing and manipulating data of a specific type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablepointer).
#[repr(transparent)]
pub struct UnsafeMutablePointer<T> {
    inner: NonNull<T>,
}

impl<T> Clone for UnsafeMutablePointer<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UnsafeMutablePointer<T> {}

impl<T> fmt::Debug for UnsafeMutablePointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> fmt::Pointer for UnsafeMutablePointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> PartialEq for UnsafeMutablePointer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for UnsafeMutablePointer<T> {}

impl<T> PartialOrd for UnsafeMutablePointer<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for UnsafeMutablePointer<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> hash::Hash for UnsafeMutablePointer<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> UnsafeMutablePointer<T> {
    /// Creates a new instance from a mutable Rust pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }

    /// Creates a new instance from a mutable Rust pointer if it is non-null.
    #[inline]
    pub fn new(ptr: *mut T) -> Option<Self> {
        Some(Self {
            inner: NonNull::new(ptr)?,
        })
    }

    /// Acquires the underlying mutable pointer.
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.inner.as_ptr()
    }

    // TODO: `as_ref` and `as_mut`
}
