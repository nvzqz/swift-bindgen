use crate::{Hashable, UnsafeRawBufferPointer};

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

impl Hasher {
    /// Adds the given value to this hasher, mixing its essential parts into the
    /// hasher state.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/hasher/2995578-combine).
    #[inline]
    pub fn combine<H: Hashable>(&mut self, value: H) {
        value.hash(self);
    }

    /// Adds the contents of the given buffer to this hasher, mixing it into the
    /// hasher state.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/hasher/2995579-combine).
    #[inline]
    pub fn combine_bytes(&mut self, bytes: &[u8]) {
        extern "C" {
            #[link_name = "$ss6HasherV7combine5bytesySW_tF"]
            fn combine(hasher: *mut Hasher, bytes: UnsafeRawBufferPointer);
        }
        unsafe { combine(self, bytes.into()) }
    }

    // TODO: Implement `finalize` by passing `Hasher` by reference into the
    // `self` register (`r13` on x86_64 and `x20` on Arm64). This requires
    // inline assembly (https://github.com/rust-lang/rust/issues/72016).
    //
    // #[inline]
    // pub fn finalize(&self) -> Int {
    //     extern "C" {
    //         #[link_name = "$ss6HasherV8finalizeSiyF"]
    //         fn finalize(hasher: Hasher) -> Int;
    //     }
    //     unsafe { finalize(*self) }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn mem_layout() {
        assert_eq!(mem::size_of::<Hasher>(), 72);
        assert_eq!(mem::align_of::<Hasher>(), 8);
    }

    #[test]
    fn default() {
        assert_eq!(Hasher::default()._core, Hasher::default()._core);
    }

    #[test]
    fn hash_bytes() {
        fn hash(bytes: &[u8]) -> Hasher {
            let mut hasher = Hasher::default();
            hasher.combine_bytes(bytes);
            hasher
        }

        for n in 0..100 {
            let bytes: Vec<u8> = (0..n).map(|_| rand::random()).collect();

            // TODO: Compare result of `finalize` instead.
            assert_eq!(hash(&bytes)._core, hash(&bytes)._core);
        }
    }
}
