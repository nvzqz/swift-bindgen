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
    ///
    /// # Panics
    ///
    /// Implementations of this function may panic if the `asm` feature is not
    /// enabled.
    fn hash(self, hasher: &mut Hasher);
}

mod sys {
    use super::*;

    #[cfg_attr(not(feature = "asm"), allow(unused))]
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

#[cfg_attr(not(feature = "asm"), allow(unused))]
unsafe impl Hashable for usize {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe {
            arch_asm! {
                "aarch64" => {
                    "bl {}",
                    sym sys::combine_UInt,
                    in("x20") hasher,
                    in("x1") self,
                }
                "x86_64" => {
                    "call {}",
                    sym sys::combine_UInt,
                    in("r13") hasher,
                    in("rsi") self,
                }
            }
        }
    }
}

unsafe impl Hashable for isize {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        (self as usize).hash(hasher);
    }
}

#[cfg_attr(not(feature = "asm"), allow(unused))]
unsafe impl Hashable for u8 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe {
            arch_asm! {
                "aarch64" => {
                    "bl {}",
                    sym sys::combine_UInt8,
                    in("x20") hasher,
                    in("x1") self,
                }
                "x86_64" => {
                    "call {}",
                    sym sys::combine_UInt8,
                    in("r13") hasher,
                    in("sil") self, // 8-bit rsi
                }
            }
        }
    }
}

unsafe impl Hashable for i8 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        (self as u8).hash(hasher);
    }
}

#[cfg_attr(not(feature = "asm"), allow(unused))]
unsafe impl Hashable for u16 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe {
            arch_asm! {
                "aarch64" => {
                    "bl {}",
                    sym sys::combine_UInt16,
                    in("x20") hasher,
                    in("x1") self,
                }
                "x86_64" => {
                    "call {}",
                    sym sys::combine_UInt16,
                    in("r13") hasher,
                    in("si") self, // 16-bit rsi
                }
            }
        }
    }
}

unsafe impl Hashable for i16 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        (self as u16).hash(hasher);
    }
}

#[cfg_attr(not(feature = "asm"), allow(unused))]
unsafe impl Hashable for u32 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe {
            arch_asm! {
                "aarch64" => {
                    "bl {}",
                    sym sys::combine_UInt32,
                    in("x20") hasher,
                    in("x1") self,
                }
                "x86_64" => {
                    "call {}",
                    sym sys::combine_UInt32,
                    in("r13") hasher,
                    in("esi") self, // 32-bit rsi
                }
            }
        }
    }
}

unsafe impl Hashable for i32 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        (self as u32).hash(hasher);
    }
}

#[cfg_attr(not(feature = "asm"), allow(unused))]
unsafe impl Hashable for u64 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        unsafe {
            arch_asm! {
                "aarch64" => {
                    "bl {}",
                    sym sys::combine_UInt64,
                    in("x20") hasher,
                    in("x1") self,
                }
                "x86_64" => {
                    "call {}",
                    sym sys::combine_UInt64,
                    in("r13") hasher,
                    in("rsi") self,
                }
            }
        }
    }
}

unsafe impl Hashable for i64 {
    #[inline]
    fn hash(self, hasher: &mut Hasher) {
        (self as u64).hash(hasher);
    }
}
