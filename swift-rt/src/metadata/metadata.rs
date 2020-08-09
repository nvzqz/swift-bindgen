use crate::{
    ctx_desc::TypeContextDescriptor,
    metadata::{EnumMetadata, MetadataKind, MetatypeMetadata, StructMetadata},
};
use std::{
    fmt,
    os::raw::{c_uint, c_void},
};
use swift_sys::metadata::{EnumValueWitnessTable, Metadata as RawMetadata, ValueWitnessTable};

/// Type metadata.
///
/// # Debug formatting
///
/// The `Debug` implementation takes into account the polymorphic nature of this
/// type. It will attempt to format the type as the specific subtype denoted by
/// the `MetadataKind`.
///
/// When emitting fields, this type and its subtypes emit the value-witness
/// table last, despite it being referenced before the metadata address in
/// memory. This is to make nested output easier to follow.
#[repr(C)]
pub struct Metadata {
    raw: RawMetadata,
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format as the specific metadata subtype.
        //
        // `fmt` is called with the type's name to ensure that the correct
        // implementation calls, and that this does not infinitely recurse.
        match self.kind() {
            MetadataKind::ENUM | MetadataKind::OPTIONAL => {
                EnumMetadata::fmt(unsafe { &*(self as *const Self as *const EnumMetadata) }, f)
            }

            MetadataKind::STRUCT => StructMetadata::fmt(
                unsafe { &*(self as *const Self as *const StructMetadata) },
                f,
            ),

            MetadataKind::METATYPE => MetatypeMetadata::fmt(
                unsafe { &*(self as *const Self as *const MetatypeMetadata) },
                f,
            ),

            // Default to "unknown" metadata.
            kind => {
                let value_witnesses: &dyn fmt::Debug = match self.enum_value_witnesses() {
                    Some(value_witnesses) => value_witnesses,
                    None => self.value_witnesses(),
                };

                f.debug_struct("UnknownMetadata")
                    .field("kind", &kind)
                    .field("value_witnesses", value_witnesses)
                    .finish()
            }
        }
    }
}

impl Metadata {
    /// Creates an instance from a raw metadata value.
    ///
    /// # Safety
    ///
    /// The resulting context where `self` is placed must be correct for the
    /// value of the raw value.
    #[inline]
    pub const unsafe fn from_raw(raw: RawMetadata) -> Self {
        Self { raw }
    }

    /// Extracts the inner raw metadata value.
    #[inline]
    pub const fn into_raw(self) -> RawMetadata {
        self.raw
    }

    /// Returns a reference to the inner raw metadata value.
    #[inline]
    pub const fn as_raw(&self) -> &RawMetadata {
        &self.raw
    }
}

impl Metadata {
    /// Creates a new metadata.
    ///
    /// # Safety
    ///
    /// The metadata context must have a memory layout appropriate for the type
    /// of metadata indicated by `kind`. This includes data that is placed
    /// immediately after or before the created instance.
    #[inline]
    pub const unsafe fn new(kind: usize) -> Self {
        Self {
            raw: RawMetadata { kind },
        }
    }

    /// Returns the kind of this metadata.
    #[inline]
    pub fn kind(&self) -> MetadataKind {
        self.raw.kind()
    }

    /// Returns the stored class isa pointer, or null if there isn't one.
    #[inline]
    pub fn isa_ptr(&self) -> *const c_void {
        self.raw.isa_ptr()
    }

    /// Returns a pointer to the value-witness table pointer from the pointer
    /// metadata.
    #[inline]
    pub(crate) fn value_witness_table_ptr(this: *const Self) -> *const *const ValueWitnessTable {
        RawMetadata::value_witness_table_ptr(this.cast()).cast()
    }

    /// Returns the value-witness table.
    #[inline]
    pub fn value_witnesses(&self) -> &ValueWitnessTable {
        unsafe { &**Self::value_witness_table_ptr(self) }
    }

    /// Returns the enum value-witness table if this metadata has enum witnesses
    /// one.
    #[inline]
    pub fn enum_value_witnesses(&self) -> Option<&EnumValueWitnessTable> {
        let ptr = Self::value_witness_table_ptr(self);

        let table_ptr = unsafe { *ptr };
        let table = unsafe { (*ptr).as_ref()? };

        if table.flags.has_enum_witnesses() {
            Some(unsafe { &*table_ptr.cast::<EnumValueWitnessTable>() })
        } else {
            None
        }
    }

    /// Returns a pointer to the type descriptor pointer from the pointer
    /// metadata.
    #[inline]
    fn type_descriptor_ptr(this: *const Self) -> *const *const TypeContextDescriptor {
        RawMetadata::type_descriptor_ptr(this.cast()).cast()
    }

    /// Returns a reference to the nominal type descriptor if this metadata
    /// represents a nominal type.
    #[inline]
    pub fn type_descriptor(&self) -> Option<&TypeContextDescriptor> {
        if self.kind().is_nominal_type() {
            unsafe { (*Self::type_descriptor_ptr(self)).as_ref() }
        } else {
            None
        }
    }
}

/// Casting to subtypes.
impl Metadata {
    /// Casts this metadata to an enum metadata if it is one.
    #[inline]
    pub fn as_enum(&self) -> Option<&EnumMetadata> {
        if self.kind().is_enum() || self.kind().is_optional() {
            Some(unsafe { &*(self as *const Self as *const EnumMetadata) })
        } else {
            None
        }
    }

    /// Casts this metadata to a struct metadata if it is one.
    #[inline]
    pub fn as_struct(&self) -> Option<&StructMetadata> {
        if self.kind().is_struct() {
            Some(unsafe { &*(self as *const Self as *const StructMetadata) })
        } else {
            None
        }
    }

    /// Casts this metadata to a metatype metadata if it is one.
    #[inline]
    pub fn as_metatype(&self) -> Option<&MetatypeMetadata> {
        if self.kind().is_metatype() {
            Some(unsafe { &*(self as *const Self as *const MetatypeMetadata) })
        } else {
            None
        }
    }
}

/// Value-witness function invocation.
///
/// # Safety
///
/// These methods call external raw C function pointers whose implementations
/// are not known to be safe. These also use raw pointers, so care must be taken
/// to ensure the read/written data is correctly referenced.
///
/// These methods are slightly safer than invoking the value witnesses directly
/// because they pass the expected metadata pointer to the `self` parameter.
impl Metadata {
    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.initialize_buffer_with_copy_of_buffer).
    #[inline(always)]
    pub unsafe fn vw_initialize_buffer_with_copy_of_buffer<T, B>(
        &self,
        dest: *mut B,
        src: *mut B,
    ) -> *mut T
    where
        B: ?Sized,
    {
        self.value_witnesses()
            .initialize_buffer_with_copy_of_buffer(dest, src, self)
    }

    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.destroy).
    #[inline(always)]
    pub unsafe fn vw_destroy<T>(&self, object: *mut T) {
        self.value_witnesses().destroy(object, self);
    }

    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.initialize_with_copy).
    #[inline(always)]
    pub unsafe fn vw_initialize_with_copy<T>(&self, dest: *mut T, src: *mut T) -> *mut T {
        self.value_witnesses().initialize_with_copy(dest, src, self)
    }

    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.assign_with_copy).
    #[inline(always)]
    pub unsafe fn vw_assign_with_copy<T>(&self, dest: *mut T, src: *mut T) -> *mut T {
        self.value_witnesses().assign_with_copy(dest, src, self)
    }

    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.initialize_with_take).
    #[inline(always)]
    pub unsafe fn vw_initialize_with_take<T>(&self, dest: *mut T, src: *mut T) -> *mut T {
        self.value_witnesses().initialize_with_take(dest, src, self)
    }

    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.assign_with_take).
    #[inline(always)]
    pub unsafe fn vw_assign_with_take<T>(&self, dest: *mut T, src: *mut T) -> *mut T {
        self.value_witnesses().assign_with_take(dest, src, self)
    }

    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.get_enum_tag_single_payload).
    #[inline(always)]
    pub unsafe fn vw_get_enum_tag_single_payload<T>(
        &self,
        enum_: *const T,
        empty_cases: c_uint,
    ) -> c_uint {
        self.value_witnesses()
            .get_enum_tag_single_payload(enum_, empty_cases, self)
    }

    /// A generic wrapper over
    /// [the value-witness function](struct.ValueWitnessTable.html#structfield.store_enum_tag_single_payload).
    #[inline(always)]
    pub unsafe fn vw_store_enum_tag_single_payload<T>(
        &self,
        enum_: *mut T,
        which_case: c_uint,
        empty_cases: c_uint,
    ) {
        self.value_witnesses()
            .store_enum_tag_single_payload(enum_, which_case, empty_cases, self);
    }
}
