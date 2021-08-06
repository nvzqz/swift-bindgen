use crate::{util::BitPattern, Comparable, Equatable, String};
use std::cmp::Ordering;

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
        pub(crate) fn eq(lhs: BitPattern<String>, rhs: BitPattern<String>) -> bool;

        #[link_name = "$sSS1loiySbSS_SStFZ"]
        pub(crate) fn lt(lhs: BitPattern<String>, rhs: BitPattern<String>) -> bool;
    }
}

impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { funcs::eq(self.into(), other.into()) }
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
        unsafe { funcs::lt(self.into(), other.into()) }
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        // Clippy suggests using `>=`, but that's what we're implementing here.
        #![allow(clippy::nonminimal_bool)]

        !(other < self)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        other < self
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        other <= self
    }
}

impl Ord for String {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // Clippy suggests using `cmp`, but that's what we're implementing here.
        #![allow(clippy::comparison_chain)]

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

    #[test]
    fn eq_repeating_empty() {
        // TODO: Test `String::repeating` against non-empty values.

        let empty = String::new();

        for count in 0..100 {
            let result = String::repeating(&empty, count);

            assert!(result == empty);
            assert!(result <= empty);
            assert!(result >= empty);
        }
    }
}
