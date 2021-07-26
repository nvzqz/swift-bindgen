use crate::{Equatable, String};
use std::{mem::ManuallyDrop, ptr};

// SAFETY: String implements Equatable.
//
// TODO: In the future, get conformance descriptor at `$sSSSQsMc`.
unsafe impl Equatable for String {}

impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // TODO: Implement this inline rather than only rely the stdlib func.

        // SAFETY: Strings are passed by-value using the C calling convention
        // for structs. The called function takes no ownership responsibility,
        // so we denote this with `ManuallyDrop`.
        //
        // On x86_64, this should pass:
        // - `%rdi` and `%rsi` from `self`
        // - `%rdx` and `%rcx` from `other`
        unsafe {
            #[link(name = "swiftCore", kind = "dylib")]
            extern "C" {
                #[link_name = "$sSS2eeoiySbSS_SStFZ"]
                fn eq(a: ManuallyDrop<String>, b: ManuallyDrop<String>) -> bool;
            }

            let a = ManuallyDrop::new(ptr::read(self));
            let b = ManuallyDrop::new(ptr::read(other));
            eq(a, b)
        }
    }
}

impl Eq for String {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_new() {
        assert!(String::new() == String::new());
    }
}
