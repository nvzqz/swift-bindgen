// TODO: Annotate as `#[repr(Swift)]`

// TODO: Figure out ABI story between `swift::Never` and `!`.

/// The return type of functions that do not return normally, that is, a type
/// with no values.
///
/// See [documentation](https://developer.apple.com/documentation/swift/never).
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Never {}

impl Never {
    /// Safely informs Rust that all subsequent code is unreachable.
    #[inline]
    pub fn consume(self) -> ! {
        match self {}
    }
}
