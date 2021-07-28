use crate::{util::BitPattern, Int};
use std::mem::MaybeUninit;
use swift_rt::metadata::{StructMetadata, Type};

mod cmp;

/// A Unicode string value that is a collection of characters.
///
/// See [documentation](https://developer.apple.com/documentation/swift/string).
#[repr(C)]
pub struct String {
    // TODO: Create separate repr types.
    raw_bits: [u64; 2],
}

impl Drop for String {
    fn drop(&mut self) {
        let metadata = Self::get_metadata().as_metadata();

        unsafe { metadata.vw_destroy(self) };
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let metadata = Self::get_metadata().as_metadata();

        unsafe {
            let mut clone = MaybeUninit::<Self>::uninit();
            metadata.vw_initialize_with_copy(clone.as_mut_ptr(), self);
            clone.assume_init()
        }
    }
}

impl Type for String {
    type Metadata = StructMetadata;

    #[inline]
    fn get_metadata<'a>() -> &'a Self::Metadata {
        extern "C" {
            #[link_name = "$sSSN"]
            static METADATA: StructMetadata;
        }

        unsafe { &METADATA }
    }

    #[inline]
    fn get_metadata_blocking<'a>(_blocking: bool) -> Option<&'a Self::Metadata> {
        Some(Self::get_metadata())
    }
}

impl Default for String {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl String {
    /// Creates a new, empty string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/string/1540864-init).
    #[inline]
    #[doc(alias = "init")]
    pub const fn new() -> Self {
        Self {
            // TODO: Create via small string repr.
            // TODO: Verify and test against big-endian and 32-bit.
            raw_bits: [0, 0xE000000000000000],
        }
    }

    /// Creates a new string representing the given string repeated the
    /// specified number of times.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/string/2427723-init).
    #[inline]
    #[doc(alias = "init(repeating:count:)")]
    pub fn repeating(value: &Self, count: Int) -> Self {
        #[link(name = "swiftCore", kind = "dylib")]
        extern "C" {
            #[link_name = "$sSS9repeating5countS2S_SitcfC"]
            fn init_repeating(value: BitPattern<String>, count: Int) -> String;
        }

        unsafe { init_repeating(value.into(), count) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_new() {
        drop(String::new());
    }

    #[test]
    fn clone_new() {
        // Test compiler-decided order.
        drop(String::new().clone());

        // Test source before clone.
        {
            let a = String::new();
            let b = a.clone();
            drop(a);
            drop(b);
        }

        // Test clone before source.
        {
            let a = String::new();
            let b = a.clone();
            drop(b);
            drop(a);
        }
    }
}
