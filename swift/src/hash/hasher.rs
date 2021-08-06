use crate::{Hashable, Int, UnsafeRawBufferPointer};
use std::mem::{self, MaybeUninit};

/// The universal hash function used by [`Set`](crate::Set) and
/// [`Dictionary`](crate::Dictionary).
///
/// See [documentation](https://developer.apple.com/documentation/swift/hasher).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Hasher {
    /// The `_Core` type comprises of two values: a buffer (1 value) and state
    /// (8 values).
    ///
    /// Currently, `Hasher` is implemented using SipHash; however, this may
    /// change in the future. So we do not expose any internals.
    _core: [u64; 9],
}

impl Default for Hasher {
    #[inline]
    #[doc(alias = "init")]
    fn default() -> Self {
        extern "C" {
            #[link_name = "$ss6HasherVABycfC"]
            fn init_hasher() -> Hasher;
        }
        unsafe { init_hasher() }
    }
}

impl core::hash::Hasher for Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.finalize() as u64
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.combine_bytes(bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.combine(i);
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.combine(i);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.combine(i);
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.combine(i);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.combine(i);
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.combine(i);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.combine(i);
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.combine(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.combine(i);
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.combine(i);
    }
}

impl Hasher {
    /// Adds the given value to this hasher, mixing its essential parts into the
    /// hasher state.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/hasher/2995578-combine).
    ///
    /// # Panics
    ///
    /// Implementations of this function may panic if the `asm` feature is not
    /// enabled.
    #[inline]
    pub fn combine<H: Hashable>(&mut self, value: H) {
        value.hash(self);
    }

    /// Adds the contents of the given buffer to this hasher, mixing it into the
    /// hasher state.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/hasher/2995579-combine).
    ///
    /// # Panics
    ///
    /// Implementations of this function may panic if the `asm` feature is not
    /// enabled.
    #[inline]
    pub fn combine_bytes(&mut self, bytes: &[u8]) {
        #[allow(unused)]
        extern "C" {
            #[link_name = "$ss6HasherV7combine5bytesySW_tF"]
            fn combine(hasher: *mut Hasher, bytes: UnsafeRawBufferPointer);
        }

        unsafe {
            #[allow(unused)]
            let [arg0, arg1]: [usize; 2] = mem::transmute(UnsafeRawBufferPointer::from(bytes));

            arch_asm! {
                "aarch64" => {
                    "bl {}",
                    sym combine,
                    in("x20") self,
                    in("x1") arg0,
                    in("x2") arg1,
                }
                "x86_64" => {
                    "call {}",
                    sym combine,
                    in("r13") self,
                    in("rsi") arg0,
                    in("rdx") arg1,
                }
            }
        }
    }

    /// Finalizes the hasher state and returns the hash value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/hasher/2995580-finalize).
    ///
    /// # Panics
    ///
    /// Implementations of this function may panic if the `asm` feature is not
    /// enabled.
    #[inline]
    pub fn finalize(self) -> Int {
        // TODO: Remove when `asm!` is stabilized.
        // See https://github.com/rust-lang/rust/issues/72016.
        #![cfg_attr(not(feature = "asm"), allow(unused, unreachable_code))]

        extern "C" {
            #[link_name = "$ss6HasherV8finalizeSiyF"]
            fn finalize(hasher: *mut Hasher) -> Int;
        }

        // `finalize` consumes the `Hasher` instance; so we assume that its
        // memory will be left uninitialized.
        let mut hasher = MaybeUninit::new(self);
        let hasher_ptr = hasher.as_mut_ptr();

        unsafe {
            let mut result: Int;

            arch_asm! {
                "aarch64" => {
                    "bl {}",
                    sym finalize,
                    in("x20") hasher_ptr,
                    out("x0") result,
                }
                "x86_64" => {
                    "call {}",
                    sym finalize,
                    in("r13") hasher_ptr,
                    out("rax") result,
                }
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[cfg(feature = "asm")]
    fn hash_i32(i: i32) -> Int {
        let mut hasher = Hasher::default();
        hasher.combine(i);
        hasher.finalize()
    }

    #[test]
    fn mem_layout() {
        assert_eq!(mem::size_of::<Hasher>(), 72);
        assert_eq!(mem::align_of::<Hasher>(), 8);
    }

    #[test]
    fn default() {
        assert_eq!(Hasher::default()._core, Hasher::default()._core);

        #[cfg(feature = "asm")]
        assert_eq!(Hasher::default().finalize(), Hasher::default().finalize());
    }

    #[test]
    #[cfg(feature = "asm")]
    fn hash_bytes() {
        fn hash(bytes: &[u8]) -> Int {
            let mut hasher = Hasher::default();
            hasher.combine_bytes(bytes);
            hasher.finalize()
        }

        for n in 0..100 {
            let bytes: Vec<u8> = (0..n).map(|_| rand::random()).collect();

            assert_eq!(hash(&bytes), hash(&bytes));
        }
    }

    #[test]
    #[cfg(feature = "asm")]
    fn different_results() {
        fn generate() -> (i32, Int) {
            let value = rand::random();
            (value, hash_i32(value))
        }

        let (mut prev_value, mut prev_result) = generate();

        for _ in 0..100 {
            let new_value = rand::random();
            let new_result = hash_i32(new_value);

            assert_ne!(
                new_result, prev_result,
                "same results for hashing {} as {}",
                new_value, prev_value
            );

            prev_value = new_value;
            prev_result = new_result;
        }
    }

    #[test]
    #[cfg(feature = "asm")]
    fn same_results() {
        for _ in 0..100 {
            let value = rand::random();
            let a = hash_i32(value);
            let b = hash_i32(value);

            assert_eq!(a, b, "different results for hashing {}", value);
        }
    }
}
