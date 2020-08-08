// TODO: Should these instead be transparent newtypes?
//
// Pros:
// - Can define custom inherent methods.
// Cons:
// - More verbose casting between types (can't use literals in-place).

/// A value type whose instances are either `true` or `false`.
///
/// See [documentation](https://developer.apple.com/documentation/swift/bool).
pub type Bool = bool;

/// A signed integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/int).
pub type Int = isize;

/// An 8-bit signed integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/int8).
pub type Int8 = i8;

/// A 16-bit signed integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/int16).
pub type Int16 = i16;

/// A 32-bit signed integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/int32).
pub type Int32 = i32;

/// A 64-bit signed integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/int64).
pub type Int64 = i64;

/// An unsigned integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/uint).
pub type UInt = usize;

/// An 8-bit unsigned integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/uint8).
pub type UInt8 = u8;

/// A 16-bit unsigned integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/uint16).
pub type UInt16 = u16;

/// A 32-bit unsigned integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/uint32).
pub type UInt32 = u32;

/// A 64-bit unsigned integer value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/uint64).
pub type UInt64 = u64;

/// A single-precision, floating-point value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/float).
pub type Float = f32;

/// A double-precision, floating-point value type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/double).
pub type Double = f64;

/// A 32-bit floating point type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/float32).
pub type Float32 = f32;

/// A 64-bit floating point type.
///
/// See [documentation](https://developer.apple.com/documentation/swift/float64).
pub type Float64 = f64;
