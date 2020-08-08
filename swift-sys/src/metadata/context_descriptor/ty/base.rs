use crate::{
    metadata::{ContextDescriptor, MetadataAccessFunction},
    ptr::{RelativeDirectPointer, RelativeDirectPointerNonNull},
    reflection::FieldDescriptor,
};
use std::os::raw::c_char;

/// Context descriptor for any nominal type.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct TypeContextDescriptor {
    /// The base context descriptor.
    pub base: ContextDescriptor,

    /// The name of the type.
    pub name: RelativeDirectPointerNonNull<c_char>,

    /// A pointer to the metadata access function for this type.
    pub access_function: RelativeDirectPointer<MetadataAccessFunction>,

    /// A pointer to the field descriptor for the type, if any.
    pub fields: RelativeDirectPointer<FieldDescriptor>,
}
