use std::{borrow::Borrow, num::NonZeroI32, ops::Deref};
use swift_sys::ptr::RelativeDirectPointerNonNull;

// TODO: Implement methods for `Offset` of `NonZeroIsize` (`intptr_t`)

/// A borrowed value that is referred at a relative offset from itself.
#[repr(transparent)]
pub struct RelativeDirect<T, Offset = NonZeroI32> {
    ptr: RelativeDirectPointerNonNull<T, Offset>,
}

impl<T> Deref for RelativeDirect<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Borrow<T> for RelativeDirect<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> AsRef<T> for RelativeDirect<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T, Offset> RelativeDirect<T, Offset> {
    /// Casts the pointer to a borrow.
    ///
    /// # Safety
    ///
    /// The placement address (`ptr`), when adjusted by the stored offset, must
    /// not:
    ///
    /// - Result in a null pointer.
    ///
    /// - Be unaligned with respect to `T`.
    #[inline]
    pub unsafe fn from_ptr(ptr: &RelativeDirectPointerNonNull<T, Offset>) -> &Self {
        &*(ptr as *const _ as *const Self)
    }

    /// Casts the borrow to a pointer.
    #[inline]
    pub fn as_ptr(this: &Self) -> &RelativeDirectPointerNonNull<T, Offset> {
        &this.ptr
    }
}
