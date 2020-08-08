use crate::ptr::RelativeDirectPointer;
use std::os::raw::c_char;

mod flags;

pub use flags::*;

/// An entry for a type's field.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FieldRecord {
    /// The flags defining the semantics of the field.
    pub flags: FieldRecordFlags,

    /// The mangled name of the field's type.
    pub mangled_type_name: RelativeDirectPointer<c_char>,

    /// The name of the field.
    pub field_name: RelativeDirectPointer<c_char>,
}
