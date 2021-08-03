use crate::{Equatable, Hasher, ObjectIdentifier};

/// A type that can be hashed into a [`Hasher`] to produce an integer hash
/// value.
///
/// See [documentation](https://developer.apple.com/documentation/swift/hashable).
///
/// # Safety
///
/// The implementation of this trait implies that there is an existing protocol
/// conformance. Types like [`Dictionary`](crate::Dictionary) take advantage of
/// this knowledge at compile-time.
pub unsafe trait Hashable: Equatable {
    /// Hashes the essential components of this value by feeding them into the
    /// given hasher.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/hashable/2995575-hash).
    fn hash(self, hasher: &mut Hasher);
}

mod sys {
    use super::*;

    extern "C" {
        #[link_name = "$ss6HasherV8_combineyySuF"]
        pub fn combine_UInt(hasher: *mut Hasher, value: usize);

        #[link_name = "$ss6HasherV8_combineyys5UInt8VF"]
        pub fn combine_UInt8(hasher: *mut Hasher, value: u8);

        #[link_name = "$ss6HasherV8_combineyys6UInt16VF"]
        pub fn combine_UInt16(hasher: *mut Hasher, value: u16);

        #[link_name = "$ss6HasherV8_combineyys6UInt32VF"]
        pub fn combine_UInt32(hasher: *mut Hasher, value: u32);

        #[link_name = "$ss6HasherV8_combineyys6UInt64VF"]
        pub fn combine_UInt64(hasher: *mut Hasher, value: u64);
    }
}

unsafe impl Hashable for ObjectIdentifier {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        self.bit_pattern().hash(hasher);
    }
}

unsafe impl Hashable for usize {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt(hasher, self) };
    }
}

unsafe impl Hashable for isize {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt(hasher, self as usize) };
    }
}

unsafe impl Hashable for u8 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt8(hasher, self) };
    }
}

unsafe impl Hashable for i8 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt8(hasher, self as u8) };
    }
}

unsafe impl Hashable for u16 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt16(hasher, self) };
    }
}

unsafe impl Hashable for i16 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt16(hasher, self as u16) };
    }
}

unsafe impl Hashable for u32 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt32(hasher, self) };
    }
}

unsafe impl Hashable for i32 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt32(hasher, self as u32) };
    }
}

unsafe impl Hashable for u64 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt64(hasher, self) };
    }
}

unsafe impl Hashable for i64 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe { sys::combine_UInt64(hasher, self as u64) };
    }
}
