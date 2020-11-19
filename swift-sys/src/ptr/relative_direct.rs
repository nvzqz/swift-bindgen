use std::{ffi::CStr, fmt, marker::PhantomData, num::NonZeroI32, os::raw::c_char, ptr, str};

// TODO: Implement methods for `Offset` of `isize` (`intptr_t`)

/// A nullable pointer whose pointee is at a relative offset from itself.
///
/// This type deliberately does not implement [`Copy`] in order to avoid
/// accidentally dereferencing from the wrong location.
#[repr(transparent)]
pub struct RelativeDirectPointer<T, Offset = i32> {
    offset: Offset,
    marker: PhantomData<*const T>,
}

impl<T, Offset: Clone> Clone for RelativeDirectPointer<T, Offset> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.offset.clone())
    }
}

impl<T, Offset: fmt::Debug> fmt::Debug for RelativeDirectPointer<T, Offset> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.offset.fmt(f)
    }
}

impl<T> fmt::Pointer for RelativeDirectPointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl<T, Offset> RelativeDirectPointer<T, Offset> {
    /// Creates a pointer whose pointee is `offset` bytes away from itself.
    #[inline]
    pub const fn new(offset: Offset) -> Self {
        Self {
            offset,
            marker: PhantomData,
        }
    }
}

impl<T> RelativeDirectPointer<T> {
    /// Creates a pointer without a pointee.
    #[inline]
    pub const fn null() -> Self {
        Self::new(0)
    }

    // An associated constant is necessary to create a generic constant.
    const NULL: Self = Self::null();

    /// A static null pointer that can be used to simplify APIs.
    #[inline]
    pub const fn null_ref<'a>() -> &'a Self {
        &Self::NULL
    }

    /// Returns the position of the pointee relative to where this pointer is
    /// stored.
    #[inline]
    pub const fn offset(&self) -> i32 {
        self.offset
    }

    /// Returns `true` if [`offset`](#method.offset) is zero.
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.offset == 0
    }

    /// Casts to a pointer of another type.
    #[inline]
    pub const fn cast<U>(self) -> RelativeDirectPointer<U> {
        RelativeDirectPointer::new(self.offset)
    }

    /// Casts to a pointer of another type without moving the instance.
    #[inline]
    pub fn cast_by_ref<U>(&self) -> &RelativeDirectPointer<U> {
        // SAFETY: Both types have the same exact ABI.
        unsafe { &*(self as *const _ as *const _) }
    }

    /// Returns the address of the pointee, or null if
    /// [`offset`](#method.offset) is zero.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        if self.is_null() {
            return ptr::null();
        }

        let start = (self as *const Self).cast::<u8>();
        start.wrapping_offset(self.offset as isize).cast()
    }

    /// Returns a reference to the value pointed to by `self`, or `None` if
    /// `offset` is zero.
    ///
    /// # Safety
    ///
    /// The placement address (`&self`), when adjusted by the stored offset,
    /// must not:
    ///
    /// - Result in a null pointer.
    ///
    /// - Be unaligned with respect to `T`.
    #[inline]
    pub unsafe fn as_ref(&self) -> Option<&T> {
        self.as_ptr().as_ref()
    }

    /// Casts to a non-null pointer.
    ///
    /// # Safety
    ///
    /// This pointer must not be null.
    #[inline]
    pub unsafe fn as_non_null(&self) -> &RelativeDirectPointerNonNull<T> {
        debug_assert!(!self.is_null(), "casted null pointer to non-null");

        &*(self as *const Self as *const _)
    }
}

impl RelativeDirectPointer<c_char> {
    /// Returns a reference to the C string pointed to by `self`, or `None` if
    /// `offset` is zero.
    ///
    /// # Safety
    ///
    /// The placement address (`&self`), when adjusted by the stored offset,
    /// must not:
    ///
    /// - Be missing a trailing zero byte.
    ///
    /// - Result in a null pointer.
    #[inline]
    pub unsafe fn as_c_str(&self) -> Option<&CStr> {
        if self.is_null() {
            None
        } else {
            Some(CStr::from_ptr(self.as_ptr()))
        }
    }

    /// Returns a reference to the UTF-8 C string pointed to by `self`, or
    /// `None` if `offset` is zero.
    ///
    /// # Safety
    ///
    /// The placement address (`&self`), when adjusted by the stored offset,
    /// must not:
    ///
    /// - Be missing a trailing zero byte.
    ///
    /// - Be invalid UTF-8.
    ///
    /// - Result in a null pointer.
    #[inline]
    pub unsafe fn as_str(&self) -> Option<&str> {
        Some(str::from_utf8_unchecked(self.as_c_str()?.to_bytes()))
    }
}

/// A non-null pointer whose pointee is at a relative offset from itself.
///
/// This type deliberately does not implement [`Copy`] in order to avoid
/// accidentally dereferencing from the wrong location.
#[repr(transparent)]
pub struct RelativeDirectPointerNonNull<T, Offset = NonZeroI32> {
    offset: Offset,
    marker: PhantomData<*const T>,
}

impl<T, Offset: Clone> Clone for RelativeDirectPointerNonNull<T, Offset> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.offset.clone())
    }
}

impl<T, Offset: fmt::Debug> fmt::Debug for RelativeDirectPointerNonNull<T, Offset> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.offset.fmt(f)
    }
}

impl<T> fmt::Pointer for RelativeDirectPointerNonNull<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl<T, Offset> RelativeDirectPointerNonNull<T, Offset> {
    /// Creates a pointer whose pointee is `offset` bytes away from itself.
    #[inline]
    pub const fn new(offset: Offset) -> Self {
        Self {
            offset,
            marker: PhantomData,
        }
    }
}

impl<T> RelativeDirectPointerNonNull<T> {
    /// Creates a pointer without checking that `offset` is non-zero.
    ///
    /// # Safety
    ///
    /// `offset` must not be zero.
    #[inline]
    pub const unsafe fn new_unchecked(offset: i32) -> Self {
        Self::new(NonZeroI32::new_unchecked(offset))
    }

    /// Returns the position of the pointee relative to where this pointer is
    /// stored.
    #[inline]
    pub const fn offset(&self) -> NonZeroI32 {
        self.offset
    }

    /// Casts to a pointer of another type.
    #[inline]
    pub const fn cast<U>(self) -> RelativeDirectPointerNonNull<U> {
        RelativeDirectPointerNonNull::new(self.offset)
    }

    /// Casts to a pointer of another type without moving the instance.
    #[inline]
    pub fn cast_by_ref<U>(&self) -> &RelativeDirectPointerNonNull<U> {
        // SAFETY: Both types have the same exact ABI.
        unsafe { &*(self as *const _ as *const _) }
    }

    /// Returns the address of the pointee.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        let start = (self as *const Self).cast::<u8>();
        start.wrapping_offset(self.offset.get() as isize).cast()
    }

    /// Returns a reference to the value pointed to by `self`.
    ///
    /// # Safety
    ///
    /// The placement address (`&self`), when adjusted by the stored offset,
    /// must not:
    ///
    /// - Result in a null pointer.
    ///
    /// - Be unaligned with respect to `T`.
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        &*self.as_ptr()
    }

    /// Casts to a nullable pointer.
    #[inline]
    pub const fn into_nullable(self) -> RelativeDirectPointer<T, i32> {
        RelativeDirectPointer {
            offset: self.offset.get(),
            marker: PhantomData,
        }
    }
}

impl RelativeDirectPointerNonNull<c_char> {
    /// Returns a reference to the C string pointed to by `self`.
    ///
    /// # Safety
    ///
    /// The placement address (`&self`), when adjusted by the stored offset,
    /// must not:
    ///
    /// - Be missing a trailing zero byte.
    ///
    /// - Result in a null pointer.
    #[inline]
    pub unsafe fn as_c_str(&self) -> &CStr {
        CStr::from_ptr(self.as_ptr())
    }

    /// Returns a reference to the UTF-8 C string pointed to by `self`.
    ///
    /// # Safety
    ///
    /// The placement address (`&self`), when adjusted by the stored offset,
    /// must not:
    ///
    /// - Be missing a trailing zero byte.
    ///
    /// - Be invalid UTF-8.
    ///
    /// - Result in a null pointer.
    #[inline]
    pub unsafe fn as_str(&self) -> &str {
        str::from_utf8_unchecked(self.as_c_str().to_bytes())
    }
}
