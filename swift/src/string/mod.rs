use crate::{util::BitPattern, Int};
use std::{ffi::CStr, mem::MaybeUninit, os::raw::c_char};
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
    #[inline]
    fn drop(&mut self) {
        let metadata = Self::get_metadata().as_metadata();

        unsafe { metadata.vw_destroy(self) };
    }
}

impl Clone for String {
    #[inline]
    fn clone(&self) -> Self {
        let metadata = Self::get_metadata().as_metadata();

        let mut clone = MaybeUninit::<Self>::uninit();
        unsafe {
            metadata.vw_initialize_with_copy(clone.as_mut_ptr(), self);
            clone.assume_init()
        }
    }
}

impl Type for String {
    type Metadata = StructMetadata;

    #[inline]
    fn get_metadata() -> &'static Self::Metadata {
        extern "C" {
            #[link_name = "$sSSN"]
            static METADATA: StructMetadata;
        }

        unsafe { &METADATA }
    }

    #[inline]
    fn get_metadata_blocking(_blocking: bool) -> Option<&'static Self::Metadata> {
        Some(Self::get_metadata())
    }
}

impl Default for String {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<char> for String {
    #[inline]
    fn from(unicode_scalar: char) -> Self {
        // TODO: Implement inline from `_uncheckedFromUTF8` implementation.

        #[link(name = "swiftCore", kind = "dylib")]
        extern "C" {
            #[link_name = "$sSSySSs7UnicodeO6ScalarVcfC"]
            fn init_unicode_scalar(unicode_scalar: u32) -> String;
        }

        unsafe { init_unicode_scalar(unicode_scalar.into()) }
    }
}

impl From<&CStr> for String {
    #[inline]
    fn from(cstr: &CStr) -> Self {
        Self::from_cstr(cstr)
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

    /// Creates a new string by copying the null-terminated UTF-8 data
    /// referenced by the given C string slice.
    ///
    /// If the given C string contains ill-formed UTF-8 code unit sequences,
    /// this initializer replaces them with the Unicode replacement character
    /// (U+FFFD).
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/string/1641523-init).
    #[inline]
    #[doc(alias = "init(cString:)")]
    pub fn from_cstr(cstr: &CStr) -> Self {
        unsafe { Self::from_cstr_ptr(cstr.as_ptr()) }
    }

    /// Creates a new string by copying the null-terminated UTF-8 data
    /// referenced by the given C string pointer.
    ///
    /// See [`String::from_cstr`] for details.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/string/1641523-init).
    #[inline]
    #[doc(alias = "init(cString:)")]
    pub unsafe fn from_cstr_ptr(cstr: *const c_char) -> Self {
        #[link(name = "swiftCore", kind = "dylib")]
        extern "C" {
            #[link_name = "$sSS7cStringSSSPys4Int8VG_tcfC"]
            fn init_cstring(cstr: *const c_char) -> String;
        }

        init_cstring(cstr)
    }

    /// Returns the number of characters in this string.
    ///
    /// Unlike Rust's [`str::len`], this does not return the number of bytes.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/string/3003522-count).
    #[inline]
    pub fn count(&self) -> Int {
        #[link(name = "swiftCore", kind = "dylib")]
        extern "C" {
            #[link_name = "$sSS5countSivg"]
            fn count(value: BitPattern<String>) -> Int;
        }

        unsafe { count(self.into()) }
    }

    /// Returns whether this string has no characters.
    ///
    /// See [documentation](https://developer.apple.com/documentation/swift/string/2946268-isempty).
    #[inline]
    #[doc(alias = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        #[link(name = "swiftCore", kind = "dylib")]
        extern "C" {
            #[link_name = "$sSS7isEmptySbvg"]
            fn is_empty(value: BitPattern<String>) -> bool;
        }
        unsafe { is_empty(self.into()) }
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

    #[test]
    fn count() {
        assert_eq!(String::new().count(), 0);

        // Test count from repeating a single character.
        let single = String::from('a');
        for n in 0..100 {
            assert_eq!(String::repeating(&single, n).count(), n);
        }
    }

    #[test]
    fn is_empty() {
        assert!(String::new().is_empty());
    }

    #[test]
    fn from_cstr() {
        let strings = ["\0", "1\0", "12\0", "123\0"];

        for &s in strings.iter() {
            let cstr = CStr::from_bytes_with_nul(s.as_bytes()).unwrap();
            let string = String::from_cstr(cstr);

            // TODO: Add `assert_eq!(string, s);` once `String` implements
            // `PartialEq<str>`. Keep the method calls since they test other
            // functionality.

            let expected_len = s.len() as Int - 1;
            assert_eq!(string.count(), expected_len);

            let expected_empty = expected_len == 0;
            assert_eq!(string.is_empty(), expected_empty);
        }
    }
}
