use swift_rt::metadata::{StructMetadata, Type};

use crate::{AnyObject, Comparable, Equatable, Int, UInt};
use std::{ffi::c_void, ptr::NonNull};

/// A unique identifier for a class instance or metatype.
///
/// See [documentation](https://developer.apple.com/documentation/swift/objectidentifier).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectIdentifier(
    // SAFETY: This is equivalent to `Builtin.RawPointer`.
    NonNull<c_void>,
);

// SAFETY: The pointer is never dereferenced and can outlive its source.
unsafe impl Send for ObjectIdentifier {}
unsafe impl Sync for ObjectIdentifier {}

impl From<&AnyObject> for ObjectIdentifier {
    #[inline]
    fn from(obj: &AnyObject) -> Self {
        Self::from_obj(obj)
    }
}

impl Type for ObjectIdentifier {
    type Metadata = StructMetadata;

    #[inline]
    fn get_metadata() -> &'static Self::Metadata {
        extern "C" {
            #[link_name = "$sSON"]
            static METADATA: StructMetadata;
        }
        unsafe { &METADATA }
    }

    #[inline]
    fn get_metadata_blocking(_blocking: bool) -> Option<&'static Self::Metadata> {
        Some(Self::get_metadata())
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }

    #[inline]
    fn is_bitwise_takable() -> bool {
        true
    }
}

unsafe impl Equatable for ObjectIdentifier {}
unsafe impl Comparable for ObjectIdentifier {}

impl ObjectIdentifier {
    /// Creates an instance that uniquely identifies the given class instance.
    ///
    /// This is equivalent to [`ObjectIdentifier.init(_: AnyObject)`][docs].
    ///
    /// Note that `obj` is passed by-reference instead of directly by-value.
    /// This is done to cheaply get its pointer representation.
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/objectidentifier/1538294-init
    #[inline]
    pub const fn from_obj(obj: &AnyObject) -> Self {
        Self(obj.as_ptr())
    }

    /// Creates an integer that captures the full value of this object
    /// identifier.
    ///
    /// This is equivalent to [`Int.init(bitPattern: ObjectIdentifier)`][docs].
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/int/2428174-init
    #[inline]
    pub fn bit_pattern(self) -> Int {
        self.0.as_ptr() as Int
    }

    /// Creates an integer that captures the full value of this object
    /// identifier.
    ///
    /// This is equivalent to [`UInt.init(bitPattern: ObjectIdentifier)`][docs].
    ///
    /// [docs]: https://developer.apple.com/documentation/swift/uint/2428142-init
    #[inline]
    pub fn unsigned_bit_pattern(self) -> UInt {
        self.0.as_ptr() as UInt
    }
}
