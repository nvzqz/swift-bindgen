use crate::{ctx_desc::TypeContextDescriptor, ptr::RelativeDirectPointer};
use std::os::raw::c_char;

/// Context descriptor for a class type.
#[repr(C)]
#[derive(Clone, Debug)]
#[rustfmt::skip] // prevent consolidating comments
pub struct ClassDescriptor {
    /// The base nominal type descriptor.
    pub base: TypeContextDescriptor,

    /// The type of the superclass, expressed as a mangled type name that can
    /// refer to the generic arguments of the subclass type.
    pub superclass_type: RelativeDirectPointer<c_char>,

    // TODO: Figure out how union fields should be exposed.

    /// If this descriptor does not have a resilient superclass, this is the
    /// negative size of metadata objects of this class (in words).
    pub metadata_negative_size_in_words: u32,

    // /// If this descriptor has a resilient superclass, this is a reference
    // /// to a cache holding the metadata's extents.
    // pub resilient_metadata_bounds: RelativeDirectPointer<StoredClassMetadataBounds>,

    /// If this descriptor does not have a resilient superclass, this is the
    /// positive size of metadata objects of this class (in words).
    pub metadata_positive_size_in_words: u32,

    // /// Otherwise, these flags are used to do things like indicating
    // /// the presence of an Objective-C resilient class stub.
    // pub metadata_positive_size_in_words: ExtraClassDescriptorFlags,

    /// The number of additional members added by this class to the class
    /// metadata. This data is opaque by default to the runtime, other than
    /// as exposed in other members; it's really just
    /// `NumImmediateMembers * sizeof(void*)` bytes of data.
    ///
    /// Whether those bytes are added before or after the address point
    /// depends on `areImmediateMembersNegative()`.
    pub num_immediate_members: u32,

    /// The number of stored properties in the class, not including its
    /// superclasses. If there is a field offset vector, this is its length.
    pub num_fields: u32,

    /// The offset of the field offset vector for this class's stored properties
    /// in its metadata, in words. 0 means there is no field offset vector.
    ///
    /// If this class has a resilient superclass, this offset is relative to the
    /// size of the resilient superclass metadata. Otherwise, it is absolute.
    pub field_offset_vector_offset: u32,
}
