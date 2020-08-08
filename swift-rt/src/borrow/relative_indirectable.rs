use std::{borrow::Borrow, num::NonZeroI32, ops::Deref};
use swift_sys::ptr::RelativeIndirectablePointerNonNull;

// TODO: Implement methods for `Offset` of `NonZeroIsize` (`intptr_t`)

/// A borrowed value that is referred either at a relative offset from itself or
/// referenced at that offset.
#[repr(transparent)]
pub struct RelativeIndirectable<T, Offset = NonZeroI32> {
    ptr: RelativeIndirectablePointerNonNull<T, Offset>,
}

impl<T> Deref for RelativeIndirectable<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Borrow<T> for RelativeIndirectable<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> AsRef<T> for RelativeIndirectable<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T, Offset> RelativeIndirectable<T, Offset> {
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
    pub unsafe fn from_ptr(ptr: &RelativeIndirectablePointerNonNull<T, Offset>) -> &Self {
        &*(ptr as *const _ as *const Self)
    }

    /// Casts the borrow to a pointer.
    #[inline]
    pub fn as_ptr(this: &Self) -> &RelativeIndirectablePointerNonNull<T, Offset> {
        &this.ptr
    }
}
