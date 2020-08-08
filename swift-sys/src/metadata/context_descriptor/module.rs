use crate::{metadata::ContextDescriptor, ptr::RelativeDirectPointerNonNull};
use std::os::raw::c_char;

/// A context descriptor for a module.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ModuleContextDescriptor {
    /// The base context descriptor.
    pub base: ContextDescriptor,

    /// The module name.
    pub name: RelativeDirectPointerNonNull<c_char>,
}
