use crate::{ptr::RelativeDirectPointer, reflection::FieldRecord};
use std::os::raw::c_char;

mod kind;

pub use kind::*;

/// A collection of field records for a single class, struct or enum
/// declaration.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FieldDescriptor {
    /// The mangled name of the type.
    pub mangled_type_name: RelativeDirectPointer<c_char>,

    /// The mangled name of the type's superclass.
    pub superclass: RelativeDirectPointer<c_char>,

    /// The kind of field being described.
    pub kind: FieldDescriptorKind,

    /// The size of a `FieldRecord`. This appears to be unchanged.
    pub field_record_size: u16,

    /// The number of fields the type has.
    pub num_fields: u32,
}

impl FieldDescriptor {
    /// Returns a pointer to the start of the field records.
    #[inline]
    pub fn field_record_start(descriptor: *const Self) -> *const FieldRecord {
        descriptor.wrapping_add(1).cast()
    }
}
