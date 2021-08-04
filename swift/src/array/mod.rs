use crate::Equatable;
use std::{ffi::c_void, marker::PhantomData, mem, ptr::NonNull};
use swift_rt::metadata::{Metadata, MetadataKind, MetadataResponse, StructMetadata, Type};
use swift_sys::{
    heap::fns as heap_fns,
    metadata::{MetadataRequest, MetadataState},
};

mod sys {
    use super::*;

    // TODO: Enable weak linking for crates that conditionally interop with
    // Swift based on its existence.
    #[link(name = "swiftCore", kind = "dylib")]
    // #[cfg(feature = "link")]
    extern "C" {
        #[link_name = "$sSaMa"]
        pub fn array_metadata_accessor(
            request: MetadataRequest,
            ty: *const Metadata,
        ) -> MetadataResponse;
    }
}

mod empty;

use empty::EmptyArray;

/// An ordered, random-access collection.
///
/// See [documentation](https://developer.apple.com/documentation/swift/array).
#[repr(transparent)]
pub struct Array<T> {
    base: NonNull<c_void>,
    marker: PhantomData<T>,
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        // SAFETY: swiftc emits a single release call.
        unsafe {
            heap_fns::swift_bridgeObjectRelease(self.base.as_ptr());
        }
    }
}

impl<T> Clone for Array<T> {
    fn clone(&self) -> Self {
        // SAFETY: swiftc emits a single retain call.
        unsafe {
            Self {
                base: NonNull::new_unchecked(heap_fns::swift_bridgeObjectRetain(
                    self.base.as_ptr(),
                )),
                marker: PhantomData,
            }
        }
    }
}

impl<T> Default for Array<T> {
    #[doc(alias = "init")]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Default for &Array<T> {
    fn default() -> Self {
        Array::empty_ref()
    }
}

impl<T> Type for Array<T>
where
    T: Type,
{
    type Metadata = StructMetadata;

    #[doc(alias = "$sSaMa")]
    fn get_metadata() -> &'static StructMetadata {
        let item_metadata = T::get_metadata().as_ref();
        let request = MetadataRequest::blocking(MetadataState::COMPLETE);

        unsafe {
            // SAFETY: The metadata accessor takes a single argument: the
            // generic item type.
            let response = sys::array_metadata_accessor(request, item_metadata);

            // Ensure the response is complete in debug builds.
            debug_assert_eq!(
                response.state(),
                MetadataState::COMPLETE,
                "incomplete metadata state for 'Swift.Array<{}>' response",
                item_metadata.name(true)
            );

            // SAFETY: Well-formed blocking requests are expected to produce a
            // complete struct metadata.
            let metadata = &*response.as_raw().value.cast::<StructMetadata>();

            // Ensure the metadata is of the expected type in debug builds.
            debug_assert_eq!(metadata.as_metadata().kind(), MetadataKind::STRUCT);

            metadata
        }
    }

    #[doc(alias = "$sSaMa")]
    fn get_metadata_blocking(blocking: bool) -> Option<&'static StructMetadata> {
        let item_metadata = T::get_metadata_blocking(blocking)?.as_ref();
        let request = MetadataRequest::new(MetadataState::COMPLETE, !blocking);

        // SAFETY: The metadata accessor takes a single argument: the generic
        // item type.
        let response = unsafe { sys::array_metadata_accessor(request, item_metadata) };

        if response.state().is_complete() {
            // SAFETY: Completed metadata is expected to be for a struct.
            let metadata = unsafe { &*response.as_raw().value.cast::<StructMetadata>() };

            // Ensure the metadata is of the expected type in debug builds.
            debug_assert_eq!(metadata.as_metadata().kind(), MetadataKind::STRUCT);

            Some(metadata)
        } else {
            None
        }
    }
}

unsafe impl<T: Type + Equatable> Equatable for Array<T> {}

impl<T: Type + Equatable> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        // SAFETY: `Equatable` implies `T` has a protocol conformance.
        unsafe { self.eq_unchecked(other) }
    }
}

impl<T: Type + Equatable + Eq> Eq for Array<T> {}

/// Non-atomic memory management.
impl<T> Array<T> {
    /// Performs a [`clone`](Self::clone) with a non-atomic retain.
    ///
    /// # Safety
    ///
    /// Because this operation is non-atomic, it may not synchronize with other
    /// threads using this object concurrently. If so, this may result in a
    /// use-after-free.
    pub unsafe fn clone_nonatomic(&self) -> Self {
        Self {
            base: NonNull::new_unchecked(heap_fns::swift_nonatomic_bridgeObjectRetain(
                self.base.as_ptr(),
            )),
            marker: PhantomData,
        }
    }

    /// Performs a [`drop`](Self::drop) with a non-atomic release.
    ///
    /// # Safety
    ///
    /// Because this operation is non-atomic, it may not synchronize with other
    /// threads using this object concurrently. If so, this may result in a
    /// use-after-free.
    pub unsafe fn drop_nonatomic(self) {
        let ptr = self.base.as_ptr();
        mem::forget(self);
        heap_fns::swift_nonatomic_bridgeObjectRelease(ptr);
    }
}

impl<T> Array<T> {
    /// Creates a new, empty array.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/array/1539784-init).
    #[doc(alias = "init")]
    pub fn new() -> Self {
        // This emits the same as what swiftc does: a single retain call to the
        // global object. It even bypasses `EMPTY_REF` since Rust knows it will
        // never change.
        Self::empty_ref().clone()
    }

    /// Returns a global reference to an empty array.
    ///
    /// This is useful for keeping an `&Array<T>` arround without multiple
    /// retain calls to the global object that represents empty arrays.
    /// [`Array::new`] is the same as calling [`clone`](Self::clone) on this
    /// value.
    pub fn empty_ref<'a>() -> &'a Self {
        // SAFETY: `EmptyArray` has the same repr as any `Array<T>`.
        unsafe { &*(EmptyArray::empty_ref() as *const _ as *const Self) }
    }
}

/// Unchecked protocol operations.
impl<T> Array<T> where T: Type {
    /// Calls the [`Equatable`] protocol conformance for `Array` without
    /// checking if the item type `T` conforms to the protocol.
    ///
    /// Use [`eq`](Self::eq) if `T` implements [`Equatable`].
    ///
    /// # Safety
    ///
    /// The generic type `T` _must_ conform to [`Equatable`] to be able to
    /// safely call the following `==` function:
    ///
    /// ```swift
    /// static (extension in Swift):Swift.Array<A where A: Swift.Equatable>.== infix([A], [A]) -> Swift.Bool
    /// ```
    pub unsafe fn eq_unchecked(&self, other: &Self) -> bool {
        // TODO: Weak linking.
        // TODO: `extern "Swift"`.
        #[link(name = "swiftCore", kind = "dylib")]
        extern "C" {
            #[link_name = "$sSasSQRzlE2eeoiySbSayxG_ABtFZ"]
            fn eq(a: *const c_void, b: *const c_void, metadata: *const Metadata) -> bool;
        }

        let metadata = Self::get_metadata().as_ref();

        eq(self.base.as_ptr(), other.base.as_ptr(), metadata)
    }

    // TODO: `gt_unchecked` that calls `Sequence.lexicographicallyPrecedes`
    // via `$sSTsSL7ElementRpzrlE25lexicographicallyPrecedesySbqd__STRd__AAQyd__ABRSlF`
    // using witness table for `$sSayxGSTsMc` (`[T]: Sequence`)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::String as SwiftString;

    #[test]
    fn drop_new() {
        drop(Array::<i32>::new());
    }

    #[test]
    fn clone_new() {
        // Test compiler-decided order.
        drop(Array::<i32>::new().clone());

        // Test source before clone.
        {
            let a = Array::<i32>::new();
            let b = a.clone();
            drop(a);
            drop(b);
        }

        // Test clone before source.
        {
            let a = Array::<i32>::new();
            let b = a.clone();
            drop(b);
            drop(a);
        }
    }

    #[test]
    fn eq_new() {
        macro_rules! imp {
            ($($ty:ty,)+) => {
                $({
                    let a = Array::<$ty>::new();
                    let b = Array::<$ty>::new();

                    assert!(a == b);
                })+
            }
        }

        // Make sure to keep this in sync with `Equatable` impls.
        imp! {
            // Primitives.
            (),
            bool,
            f32, f64,
            u8, u16, u32, u64, usize,
            i8, i16, i32, i64, isize,

            // Standard library types.
            SwiftString,
        }
    }

    #[test]
    fn metadata_name() {
        fn test<T: Type>(name: &str) {
            {
                let expected_name = format!("Swift.Array<{}>", name);

                let metadata: &Metadata = Array::<T>::get_metadata().as_ref();
                let metadata_name = metadata.name(true);

                assert_eq!(expected_name, metadata_name);
            }

            {
                let expected_name = format!("Swift.Array<Swift.Array<{}>>", name);

                let metadata: &Metadata = Array::<Array<T>>::get_metadata().as_ref();
                let metadata_name = metadata.name(true);

                assert_eq!(expected_name, metadata_name);
            }
        }

        // Primitives.
        test::<()>("()");
        test::<bool>("Swift.Bool");
        test::<isize>("Swift.Int");
        test::<usize>("Swift.UInt");
        test::<i64>("Swift.Int64");
        test::<u64>("Swift.UInt64");
        test::<f32>("Swift.Float");
        test::<f64>("Swift.Double");

        // Standard library types.
        test::<SwiftString>("Swift.String");
    }
}
