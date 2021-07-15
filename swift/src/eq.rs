/// A type that can be compared for value equality.
///
/// This trait is similar to [`PartialEq`] in that it requires an equality
/// function to be implemented, and so it requires [`PartialEq`]. This trait
/// _does not_ require [`Eq`] because.
///
/// See [documentation](https://developer.apple.com/documentation/swift/equatable).
///
/// # Safety
///
/// The implementation of this trait implies that there is an existing protocol
/// conformance.
pub unsafe trait Equatable: PartialEq {}

macro_rules! imp {
    ($($ty:ty,)+) => {
        $(unsafe impl Equatable for $ty {})+
    };
}

imp! {
    (),
    bool,
    f32, f64,
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
}
