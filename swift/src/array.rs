use std::{cell::UnsafeCell, ffi::c_void, marker::PhantomData, ptr::NonNull};
use swift_rt::metadata::{Metadata, MetadataKind, MetadataResponse, StructMetadata, Type};
use swift_sys::{
    heap::fns as heap_fns,
    metadata::{MetadataRequest, MetadataState},
};

mod sys {
    use super::*;

    #[repr(C)]
    pub struct EmptyArrayStorage {
        opaque: [u8; 0],
    }

    // TODO: Enable weak linking for crates that conditionally interop with
    // Swift based on its existence.
    #[link(name = "swiftCore", kind = "dylib")]
    // #[cfg(feature = "link")]
    extern "C" {
        pub static _swiftEmptyArrayStorage: UnsafeCell<EmptyArrayStorage>;

        #[link_name = "$sSaMa"]
        pub fn array_metadata_accessor(
            request: MetadataRequest,
            ty: *const Metadata,
        ) -> MetadataResponse;
    }
}

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
    fn get_metadata<'a>() -> &'a StructMetadata {
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
    fn get_metadata_blocking<'a>(blocking: bool) -> Option<&'a StructMetadata> {
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
        #[repr(transparent)]
        struct EmptyArray {
            #[allow(unused)]
            base: NonNull<sys::EmptyArrayStorage>,
        }

        // Required for creating a `static` instance.
        unsafe impl Sync for EmptyArray {}

        static EMPTY: EmptyArray = unsafe {
            EmptyArray {
                base: NonNull::new_unchecked(sys::_swiftEmptyArrayStorage.get()),
            }
        };

        // SAFETY: `EmptyArray` has the same repr as any `Array<T>`.
        unsafe { &*(&EMPTY as *const _ as *const Self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        test::<()>("()");
        test::<bool>("Swift.Bool");
        test::<isize>("Swift.Int");
        test::<usize>("Swift.UInt");
        test::<i64>("Swift.Int64");
        test::<u64>("Swift.UInt64");
        test::<f32>("Swift.Float");
        test::<f64>("Swift.Double");
    }
}
