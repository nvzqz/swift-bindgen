use crate::Equatable;

/// A type that can be compared using the relational operators `<`, `<=`, `>=`,
/// and `>`.
///
/// This trait is similar to [`PartialOrd`] in that it requires comparison
/// functions to be implemented, and so it requires [`PartialOrd`]. This trait
/// _does not_ require [`Ord`] because `Comparable` does not require
/// [total order](https://en.wikipedia.org/wiki/Total_order); only
/// [partial order](https://en.wikipedia.org/wiki/Partial_order).
///
/// See [documentation](https://developer.apple.com/documentation/swift/comparable).
///
/// # Safety
///
/// The implementation of this trait implies that there is an existing protocol
/// conformance. Types like [`Array`](crate::Array) take advantage of this
/// knowledge at compile-time.
pub unsafe trait Comparable: Equatable + PartialOrd {}

macro_rules! imp {
    ($($ty:ty,)+) => {
        $(unsafe impl Comparable for $ty {})+
    };
}

imp! {
    (),
    bool,
    f32, f64,
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
}
