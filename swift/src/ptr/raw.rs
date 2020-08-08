use std::{ffi::c_void, fmt, ptr::NonNull};

/// A raw pointer for accessing untyped data.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsaferawpointer).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnsafeRawPointer {
    inner: NonNull<c_void>,
}

impl fmt::Debug for UnsafeRawPointer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Pointer for UnsafeRawPointer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl UnsafeRawPointer {
    /// Creates a new instance from an immutable Rust pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *const c_void) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr as *mut c_void),
        }
    }

    /// Creates a new instance from an immutable Rust pointer if it is non-null.
    #[inline]
    pub fn new(ptr: *const c_void) -> Option<Self> {
        Some(Self {
            inner: NonNull::new(ptr as *mut c_void)?,
        })
    }

    /// Acquires the underlying immutable pointer.
    #[inline]
    pub const fn as_ptr(self) -> *const c_void {
        self.inner.as_ptr()
    }
}

/// A raw pointer for accessing and manipulating untyped data.
///
/// See [documentation](https://developer.apple.com/documentation/swift/unsafemutablerawpointer).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnsafeMutableRawPointer {
    inner: NonNull<c_void>,
}

impl fmt::Debug for UnsafeMutableRawPointer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Pointer for UnsafeMutableRawPointer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl UnsafeMutableRawPointer {
    /// Creates a new instance from a mutable Rust pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut c_void) -> Self {
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }

    /// Creates a new instance from a mutable Rust pointer if it is non-null.
    #[inline]
    pub fn new(ptr: *mut c_void) -> Option<Self> {
        Some(Self {
            inner: NonNull::new(ptr)?,
        })
    }

    /// Acquires the underlying mutable pointer.
    #[inline]
    pub const fn as_ptr(self) -> *mut c_void {
        self.inner.as_ptr()
    }
}
