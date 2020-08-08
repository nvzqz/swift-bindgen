use std::{ffi::c_void, fmt};

/// A utility type for conveniently formatting nullable objects and pointers in
/// the style of Swift's formatting.
#[derive(Clone, Copy)]
pub(crate) struct Nil;

impl fmt::Debug for Nil {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("nil")
    }
}

impl Nil {
    /// Returns `ptr` as a `Debug` impl, or a reference to a `Nil` if `ptr` is
    /// null.
    ///
    /// This method takes care to only use the dynamic dispatch table for
    /// `*const c_void` regardless of `T`. This reduces binary size and improves
    /// cache efficiency.
    #[inline]
    pub fn fmt_debug_ptr<T>(ptr: &*const T) -> &dyn fmt::Debug {
        if ptr.is_null() {
            &Nil
        } else {
            // SAFETY: Raw pointers to `Sized` types have the same memory layout
            // regardless of the pointee type.
            unsafe { &*(ptr as *const *const T as *const *const c_void) }
        }
    }

    /// Returns `ptr` as a `Debug` impl, or a reference to a `Nil` if `ptr` is
    /// null.
    ///
    /// This method takes care to only use the dynamic dispatch table for
    /// `*const c_void` regardless of `T`. This reduces binary size and improves
    /// cache efficiency.
    #[inline]
    pub fn fmt_debug_mut_ptr<T>(ptr: &*mut T) -> &dyn fmt::Debug {
        // SAFETY: Raw pointers have the same memory layout regardless of
        // mutability.
        let ptr = unsafe { &*(ptr as *const *mut T as *const *const T) };

        Self::fmt_debug_ptr(ptr)
    }
}
