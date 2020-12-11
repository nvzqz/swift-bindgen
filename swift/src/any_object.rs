use crate::{AnyClass, AnyType};
use std::{ffi::c_void, mem, ptr::NonNull};
use swift_sys::{casting::*, heap::fns::*};

// TODO: Make `AnyObject` work with `Arc` from https://github.com/nvzqz/fruity.

/// The protocol to which all classes implicitly conform.
///
/// See [documentation](https://developer.apple.com/documentation/swift/anyobject).
///
/// # Discussion
///
/// You use `AnyObject` when you need the flexibility of an untyped object or
/// when you use bridged Objective-C methods and properties that return an
/// untyped result. `AnyObject` can be used as the concrete type for an instance
/// of any class, class type, or class-only protocol.
///
/// For example:
///
/// ```swift
/// class FloatRef {
///     let value: Float
///     init(_ value: Float) {
///         self.value = value
///     }
/// }
///
/// let x = FloatRef(2.3)
/// let y: AnyObject = x
/// let z: AnyObject = FloatRef.self
/// ```
#[repr(transparent)]
pub struct AnyObject {
    // TODO: Use pointer type that takes advantage of
    // `_swift_abi_LeastValidPointerValue`.
    ptr: NonNull<c_void>,
}

impl Drop for AnyObject {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            swift_unknownObjectRelease(self.ptr.as_ptr());
        }
    }
}

impl Clone for AnyObject {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(swift_unknownObjectRetain(self.ptr.as_ptr())) },
        }
    }
}

/// Non-atomic memory management.
impl AnyObject {
    // TODO: Create `Rc` in https://github.com/nvzqz/fruity to use for these.

    /// Performs a [`clone`](Self::clone) with a non-atomic retain.
    ///
    /// # Safety
    ///
    /// Because this operation is non-atomic, it may not synchronize with other
    /// threads using this object concurrently. If so, this may result in a
    /// use-after-free.
    #[inline]
    pub unsafe fn clone_nonatomic(&self) -> Self {
        Self {
            ptr: NonNull::new_unchecked(swift_nonatomic_unknownObjectRetain(self.ptr.as_ptr())),
        }
    }

    /// Performs a [`drop`](Self::drop) with a non-atomic release.
    ///
    /// # Safety
    ///
    /// Because this operation is non-atomic, it may not synchronize with other
    /// threads using this object concurrently. If so, this may result in a
    /// use-after-free.
    #[inline]
    pub unsafe fn drop_nonatomic(self) {
        let ptr = self.ptr.as_ptr();
        mem::forget(self);
        swift_nonatomic_unknownObjectRelease(ptr);
    }
}

impl AnyObject {
    #[inline]
    pub(crate) const fn as_ptr(&self) -> NonNull<c_void> {
        self.ptr
    }

    /// Returns the dynamic type of this object.
    ///
    /// This is equivalent to [`type(of:)`][docs].
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/2885064-type
    #[inline]
    pub fn get_type(&self) -> AnyType {
        self.get_class().into()
    }

    /// Returns the dynamic type of this object.
    ///
    /// This is equivalent to [`type(of:)`][docs].
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/2885064-type
    #[inline]
    pub fn get_class(&self) -> AnyClass {
        unsafe {
            let ty = swift_getObjectType(self.as_ptr().as_ptr().cast());
            AnyClass::from_metadata(NonNull::new_unchecked(ty as *mut _))
        }
    }
}
