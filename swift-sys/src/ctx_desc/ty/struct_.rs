use crate::ctx_desc::TypeContextDescriptor;

/// Context descriptor for a struct type.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct StructDescriptor {
    /// The base nominal type descriptor.
    pub base: TypeContextDescriptor,

    /// The number of stored properties in the struct. If there is a field
    /// offset vector, this is its length.
    pub num_fields: u32,

    /// The offset of the field offset vector for this struct's stored
    /// properties in its metadata, if any. 0 means there is no field offset
    /// vector.
    pub field_offset_vector_offset: u32,
}

impl StructDescriptor {
    /// Returns `true` if metadata records for this type have a field offset
    /// vector for its stored properties.
    #[inline]
    pub fn has_field_offset_vector(&self) -> bool {
        self.field_offset_vector_offset != 0
    }
}
