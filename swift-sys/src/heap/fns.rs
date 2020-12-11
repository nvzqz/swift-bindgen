//! Public runtime functions.

#![cfg(feature = "link")]

use std::os::raw::{c_int, c_void};

// TODO: Enable weak linking for crates that conditionally interop with Swift
// based on its existence.
#[link(name = "swiftCore", kind = "dylib")]
extern "C" {
    /// Increment the strong retain count of an object which might not be a
    /// native Swift object.
    pub fn swift_unknownObjectRetain(obj: *mut c_void) -> *mut c_void;

    /// Increment the strong retain count of an object which might not be a
    /// native Swift object by `n`.
    pub fn swift_unknownObjectRetain_n(obj: *mut c_void, n: c_int) -> *mut c_void;

    /// Increment the strong retain count of an object which might not be a
    /// native Swift object.
    pub fn swift_nonatomic_unknownObjectRetain(obj: *mut c_void) -> *mut c_void;

    /// Increment the strong retain count of an object which might not be a
    /// native Swift object by `n`.
    pub fn swift_nonatomic_unknownObjectRetain_n(obj: *mut c_void, n: c_int) -> *mut c_void;

    /// Decrement the strong retain count of an object which might not be a
    /// native Swift object.
    pub fn swift_unknownObjectRelease(obj: *mut c_void);

    /// Decrement the strong retain count of an object which might not be a
    /// native Swift object by `n`.
    pub fn swift_unknownObjectRelease_n(obj: *mut c_void);

    /// Decrement the strong retain count of an object which might not be a
    /// native Swift object.
    pub fn swift_nonatomic_unknownObjectRelease(obj: *mut c_void);

    /// Decrement the strong retain count of an object which might not be a
    /// native Swift object by `n`.
    pub fn swift_nonatomic_unknownObjectRelease_n(obj: *mut c_void);
}
