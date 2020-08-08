use std::{fmt, marker::PhantomData, mem, num::NonZeroI32};

/// A nullable pointer whose pointee is either at a relative offset from itself
/// or referenced at that offset.
///
/// This type deliberately does not implement
/// [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) in order to
/// avoid accidentally dereferencing from the wrong location.
#[repr(transparent)]
pub struct RelativeIndirectablePointer<T, Offset = i32> {
    offset: Offset,
    marker: PhantomData<*const T>,
}

impl<T, Offset: Clone> Clone for RelativeIndirectablePointer<T, Offset> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.offset.clone())
    }
}

impl<T> fmt::Debug for RelativeIndirectablePointer<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let offset = self.offset & !1;
        let name = if self.is_direct() {
            "Direct"
        } else {
            "Indirect"
        };
        f.debug_tuple(name).field(&offset).finish()
    }
}

impl<T, Offset> RelativeIndirectablePointer<T, Offset> {
    /// Creates a pointer whose pointee is either `offset` bytes away from
    /// itself or behind another pointer that's `offset` bytes away.
    ///
    /// If the low bit of `offset` is set, then the pointer references an
    /// indirect address. Otherwise, it is a direct address.
    #[inline]
    pub const fn new(offset: Offset) -> Self {
        Self {
            offset,
            marker: PhantomData,
        }
    }
}

impl<T> RelativeIndirectablePointer<T> {
    /// Creates a direct pointer to the null address.
    #[inline]
    pub const fn null() -> Self {
        Self::new(0)
    }

    const NULL: Self = Self::null();

    /// A static null pointer that can be used to simplify APIs.
    #[inline]
    pub const fn null_ref<'a>() -> &'a Self {
        &Self::NULL
    }

    /// Returns the stored offset of `self`.
    #[inline]
    pub const fn offset(&self) -> i32 {
        self.offset
    }

    /// Returns `true` if this has a zero relative offset.
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.offset == 0
    }

    /// Returns `true` if the address of `&self` adjusted by `offset` refers
    /// directly to the pointee.
    #[inline]
    pub const fn is_direct(&self) -> bool {
        !self.is_indirect()
    }

    /// Returns `true` if the address of `&self` adjusted by `offset` refers
    /// a pointer to the pointee.
    #[inline]
    pub const fn is_indirect(&self) -> bool {
        self.offset & 1 != 0
    }

    /// Casts `self` to a pointer of another type.
    #[inline]
    pub const fn cast<U>(self) -> RelativeIndirectablePointer<U> {
        RelativeIndirectablePointer::new(self.offset)
    }

    /// Casts `self` to a pointer of another type without moving.
    #[inline]
    pub fn cast_by_ref<U>(&self) -> &RelativeIndirectablePointer<U> {
        // SAFETY: Both types have the same exact ABI.
        unsafe { &*(self as *const _ as *const _) }
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
        // This this gets optimized away if it passes, because the condition can
        // be evaluated at compile-time.
        assert!(
            mem::align_of::<T>() >= 2,
            "alignment of value must be at least 2 to make room for indirectable flag"
        );

        if self.is_null() {
            return None;
        }

        let start = (self as *const Self).cast::<u8>();
        let address = start.wrapping_offset(self.offset as isize & !1);

        // If the low bit is set, then this is an indirect address. Otherwise,
        // it's direct.
        if self.is_direct() {
            Some(&*address.cast::<T>())
        } else {
            Some(&**address.cast::<*const T>())
        }
    }

    /// Casts to a non-null pointer.
    ///
    /// # Safety
    ///
    /// This pointer must not be null.
    #[inline]
    pub unsafe fn as_non_null(&self) -> &RelativeIndirectablePointerNonNull<T> {
        debug_assert!(!self.is_null(), "casted null pointer to non-null");

        &*(self as *const Self as *const _)
    }
}

/// A non-null pointer whose pointee is either at a relative offset from itself
/// or referenced at that offset.
///
/// This type deliberately does not implement
/// [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) in order to
/// avoid accidentally dereferencing from the wrong location.
#[repr(transparent)]
pub struct RelativeIndirectablePointerNonNull<T, Offset = NonZeroI32> {
    offset: Offset,
    marker: PhantomData<*const T>,
}

impl<T, Offset: Clone> Clone for RelativeIndirectablePointerNonNull<T, Offset> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.offset.clone())
    }
}

impl<T> fmt::Debug for RelativeIndirectablePointerNonNull<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let offset = self.offset.get() & !1;
        let name = if self.is_direct() {
            "Direct"
        } else {
            "Indirect"
        };
        f.debug_tuple(name).field(&offset).finish()
    }
}

impl<T, Offset> RelativeIndirectablePointerNonNull<T, Offset> {
    /// Creates a pointer whose pointee is either `offset` bytes away from
    /// itself or behind another pointer that's `offset` bytes away.
    ///
    /// If the low bit of `offset` is set, then the pointer references an
    /// indirect address. Otherwise, it is a direct address.
    #[inline]
    pub const fn new(offset: Offset) -> Self {
        Self {
            offset,
            marker: PhantomData,
        }
    }
}

impl<T> RelativeIndirectablePointerNonNull<T> {
    /// Creates a pointer without checking that `offset` is non-zero.
    ///
    /// # Safety
    ///
    /// `offset` must not be zero.
    #[inline]
    pub const unsafe fn new_unchecked(offset: i32) -> Self {
        Self::new(NonZeroI32::new_unchecked(offset))
    }

    /// Returns the stored offset of `self`.
    #[inline]
    pub const fn offset(&self) -> NonZeroI32 {
        self.offset
    }

    /// Returns `true` if the address of `&self` adjusted by `offset` refers
    /// directly to the pointee.
    #[inline]
    pub const fn is_direct(&self) -> bool {
        !self.is_indirect()
    }

    /// Returns `true` if the address of `&self` adjusted by `offset` refers
    /// a pointer to the pointee.
    #[inline]
    pub const fn is_indirect(&self) -> bool {
        self.offset.get() & 1 != 0
    }

    /// Casts `self` to a pointer of another type.
    #[inline]
    pub const fn cast<U>(self) -> RelativeIndirectablePointerNonNull<U> {
        RelativeIndirectablePointerNonNull::new(self.offset)
    }

    /// Casts `self` to a pointer of another type without moving.
    #[inline]
    pub fn cast_by_ref<U>(&self) -> &RelativeIndirectablePointerNonNull<U> {
        // SAFETY: Both types have the same exact ABI.
        unsafe { &*(self as *const _ as *const _) }
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
        // This this gets optimized away if it passes, because the condition can
        // be evaluated at compile-time.
        assert!(
            mem::align_of::<T>() >= 2,
            "alignment of value must be at least 2 to make room for indirectable flag"
        );

        let start = (self as *const Self).cast::<u8>();
        let address = start.wrapping_offset(self.offset.get() as isize & !1);

        // If the low bit is set, then this is an indirect address. Otherwise,
        // it's direct.
        if self.is_direct() {
            &*address.cast::<T>()
        } else {
            &**address.cast::<*const T>()
        }
    }

    /// Casts to a nullable pointer.
    #[inline]
    pub const fn into_nullable(self) -> RelativeIndirectablePointer<T, i32> {
        RelativeIndirectablePointer {
            offset: self.offset.get(),
            marker: PhantomData,
        }
    }
}
