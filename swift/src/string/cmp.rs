use crate::{Comparable, Equatable, String};
use std::{cmp::Ordering, mem::ManuallyDrop, ptr};

// SAFETY: String implements Equatable and Comparable.
//
// TODO: Get conformance descriptors at `$sSSSQsMc` and `$sSSSLsMc`.
unsafe impl Equatable for String {}
unsafe impl Comparable for String {}

// TODO: Implement these inline rather than only rely the stdlib funcs.
mod funcs {
    use super::*;

    // SAFETY: Strings are passed by-value using the C calling convention for
    // structs. The called function takes no ownership responsibility, so we
    // denote this with `ManuallyDrop`.
    //
    // On x86_64, this should pass:
    // - `%rdi` and `%rsi` from `lhs`
    // - `%rdx` and `%rcx` from `rhs`
    #[link(name = "swiftCore", kind = "dylib")]
    extern "C" {
        // Only `==` and `<` infix funcs are available. All other operations are
        // implemented in terms of these.

        #[link_name = "$sSS2eeoiySbSS_SStFZ"]
        pub fn eq(lhs: ManuallyDrop<String>, rhs: ManuallyDrop<String>) -> bool;

        #[link_name = "$sSS1loiySbSS_SStFZ"]
        pub fn lt(lhs: ManuallyDrop<String>, rhs: ManuallyDrop<String>) -> bool;
    }
}

impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let lhs = ManuallyDrop::new(ptr::read(self));
            let rhs = ManuallyDrop::new(ptr::read(other));
            funcs::eq(lhs, rhs)
        }
    }
}

impl Eq for String {}

impl PartialOrd for String {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        unsafe {
            let lhs = ManuallyDrop::new(ptr::read(self));
            let rhs = ManuallyDrop::new(ptr::read(other));
            funcs::lt(lhs, rhs)
        }
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        !(other < self)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        other > self
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        !(self < other)
    }
}

impl Ord for String {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_new() {
        assert!(String::new() == String::new());
        assert!(String::new() <= String::new());
        assert!(String::new() >= String::new());
    }
}
