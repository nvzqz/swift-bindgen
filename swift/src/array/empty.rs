use std::{cell::UnsafeCell, ptr::NonNull};

mod sys {
    use super::*;

    #[repr(C)]
    pub struct EmptyArrayStorage {
        opaque: [u8; 0],
    }

    #[link(name = "swiftCore", kind = "dylib")]
    extern "C" {
        pub static _swiftEmptyArrayStorage: UnsafeCell<EmptyArrayStorage>;
    }
}

#[repr(transparent)]
pub(crate) struct EmptyArray {
    #[allow(unused)]
    base: NonNull<sys::EmptyArrayStorage>,
}

// Required for creating a `static` instance.
unsafe impl Sync for EmptyArray {}

impl EmptyArray {
    #[inline]
    pub fn empty_ref() -> &'static Self {
        static EMPTY: EmptyArray = unsafe {
            EmptyArray {
                base: NonNull::new_unchecked(sys::_swiftEmptyArrayStorage.get()),
            }
        };

        &EMPTY
    }
}
