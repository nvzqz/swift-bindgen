use std::{mem::ManuallyDrop, ptr};

/// A value that represents the valid bits of some type `T` and using the same
/// memory representation.
///
/// This is used to help pass FFI values by-value with the correct semantics.
#[repr(transparent)]
pub(crate) struct BitPattern<T> {
    #[allow(dead_code)]
    value: ManuallyDrop<T>,
}

impl<T> Clone for BitPattern<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { ptr::read(self) }
    }
}

impl<T> From<&T> for BitPattern<T> {
    #[inline]
    fn from(value: &T) -> Self {
        Self {
            value: ManuallyDrop::new(unsafe { ptr::read(value) }),
        }
    }
}
