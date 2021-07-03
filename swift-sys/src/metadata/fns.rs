//! Public runtime functions.

// #![cfg(feature = "link")]

use crate::metadata::Metadata;
use std::os::raw::c_char;

/// The pair of values returned by type name lookup functions.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub struct TypeNamePair {
    pub data: *const c_char,
    pub length: usize,
}

// TODO: Enable weak linking for crates that conditionally interop with Swift
// based on its existence.
//
// TODO: Support the Swift calling convention in rustc
// See https://github.com/rust-lang/rust/pull/64582
#[link(name = "swiftCore", kind = "dylib")]
extern /* "Swift" */ {
    /// Returns the name of a Swift type represented by a metadata object.
    pub fn swift_getTypeName(ty: *const Metadata, qualified: bool) -> TypeNamePair;

    /// Returns the mangled name of a Swift type represented by a metadata
    /// object.
    ///
    /// # Availability
    ///
    /// **Swift:** 5.3
    pub fn swift_getMangledTypeName(ty: *const Metadata) -> TypeNamePair;
}
