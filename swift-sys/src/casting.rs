//! Runtime functions for casting values.

#![cfg(feature = "link")]

use crate::{metadata::Metadata, OpaqueValue};

extern "C" {
    /// Returns `true` if the metadata is for a class type.
    pub fn swift_isClassType(ty: *const Metadata) -> bool;

    /// Returns `true` if the metadata is for an `Optional<T>` type.
    pub fn swift_isOptionalType(ty: *const Metadata) -> bool;

    /// Fetch the type metadata associated with the formal dynamic type of the
    /// given (possibly Objective-C) object. The formal dynamic type ignores
    /// dynamic subclasses such as those introduced by KVO.
    ///
    /// The object pointer may be a tagged pointer, but cannot be null.
    pub fn swift_getObjectType(obj: *const OpaqueValue) -> *const Metadata;
}
