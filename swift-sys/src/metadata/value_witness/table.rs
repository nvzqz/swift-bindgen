use crate::{mem::MemoryLayout, metadata::ValueWitnessFlags};
use std::os::raw::{c_uint, c_void};

/// A vtable of functions that implement value semantics of a type.
///
/// Provides fundamental operations such as allocating, copying, and destroying
/// values of the type. The value witness table also records the size,
/// alignment, stride, and other fundamental properties of the type.
///
/// Equivalent to fields in `#if WANT_REQUIRED_VALUE_WITNESSES` in
/// [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def).
/// Fields for `#if WANT_ENUM_VALUE_WITNESSES` are in
/// [`EnumValueWitnessTable`](struct.EnumValueWitnessTable.html).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ValueWitnessTable {
    /// Given an invalid buffer `dest`, initialize it as a copy of the object in
    /// the `src` buffer.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// T *(*initializeBufferWithCopyOfBuffer)(B *dest, B *src, M *self);
    /// ```
    pub initialize_buffer_with_copy_of_buffer: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *const c_void,
    ) -> *mut c_void,

    /// Given a valid object of this type, destroy it, leaving it as an invalid
    /// object. This is useful when generically destroying an object which has
    /// been allocated in-line, such as an array, struct, or tuple element.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// void (*destroy)(T *object, witness_t *self);
    /// ```
    pub destroy: unsafe extern "C" fn(object: *mut c_void, self_: *const c_void),

    /// Given an invalid object of this type, initialize it as a copy of the
    /// `src` object. Returns the `dest` object.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// T *(*initializeWithCopy)(T *dest, T *src, M *self);
    /// ```
    pub initialize_with_copy: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *const c_void,
    ) -> *mut c_void,

    /// Given a valid object of this type, change it to be a copy of the `src`
    /// object. Returns the `dest` object.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// T *(*assignWithCopy)(T *dest, T *src, M *self);
    /// ```
    pub assign_with_copy: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *const c_void,
    ) -> *mut c_void,

    /// Given an invalid object of this type, initialize it by taking the value
    /// of the source object. The `src` object becomes invalid. Returns the
    /// `dest` object.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// T *(*initializeWithTake)(T *dest, T *src, M *self);
    /// ```
    pub initialize_with_take: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *const c_void,
    ) -> *mut c_void,

    /// Given a valid object of this type, change it to be a copy of the `src`
    /// object. The source object becomes invalid. Returns the `dest` object.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// T *(*assignWithTake)(T *dest, T *src, M *self);
    /// ```
    pub assign_with_take: unsafe extern "C" fn(
        dest: *mut c_void,
        src: *mut c_void,
        self_: *const c_void,
    ) -> *mut c_void,

    /// Given an instance of valid single payload enum with a payload of this
    /// witness table's type (e.g `Optional<ThisType>`), get the tag of the
    /// enum.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// unsigned (*getEnumTagSinglePayload)(const T* enum,
    ///                                     UINT_TYPE emptyCases
    ///                                     M* self);
    /// ```
    pub get_enum_tag_single_payload: unsafe extern "C" fn(
        enum_: *const c_void,
        empty_cases: c_uint,
        self_: *const c_void,
    ) -> c_uint,

    /// Given uninitialized memory for an instance of a single payload enum with
    /// a payload of this witness table's type (e.g `Optional<ThisType>`), store
    /// the tag.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// void (*storeEnumTagSinglePayload)(T* enum,
    ///                                   UINT_TYPE whichCase,
    ///                                   UINT_TYPE emptyCases,
    ///                                   M* self);
    /// ```
    pub store_enum_tag_single_payload: unsafe extern "C" fn(
        enum_: *mut c_void,
        which_case: c_uint,
        empty_cases: c_uint,
        self_: *const c_void,
    ),

    /// The required storage size of a single object of this type.
    pub size: usize,

    /// The required size per element of an array of this type. It is at least
    /// one, even for zero-sized types, like the empty tuple.
    pub stride: usize,

    /// Extra information about type layout and value semantics.
    pub flags: ValueWitnessFlags,

    /// The number of extra inhabitants in the type.
    pub extra_inhabitant_count: c_uint,
}

/// Methods that wrap [`ValueWitnessFlags`](struct.ValueWitnessFlags.html).
impl ValueWitnessTable {
    /// Returns `true` if the value is allocated inline.
    #[inline]
    pub const fn is_value_inline(&self) -> bool {
        self.flags.is_inline_storage()
    }

    /// Returns `true` if values of this type can be copied with `memcpy` and
    /// destroyed with a no-op.
    #[inline]
    pub const fn is_pod(&self) -> bool {
        self.flags.is_pod()
    }

    /// Returns `true` if values of this type can be taken with `memcpy`.
    #[inline]
    pub const fn is_bitwise_takable(&self) -> bool {
        self.flags.is_bitwise_takable()
    }
}

impl ValueWitnessTable {
    /// Returns the size, stride, and alignment of the type.
    #[inline]
    pub const fn memory_layout(&self) -> MemoryLayout {
        MemoryLayout {
            size: self.size,
            stride: self.stride,
            align: self.flags.align(),
        }
    }
}

/// Function pointer wrapper methods.
///
/// These methods use generic parameters to:
///
/// - Make the function pointers consistent in argument and return types, making
///   it harder to misuse them.
///
/// - Simplify calling code by being able to take advantage of type inference.
impl ValueWitnessTable {
    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.initialize_buffer_with_copy_of_buffer).
    #[inline(always)]
    pub unsafe fn initialize_buffer_with_copy_of_buffer<M, T, B>(
        &self,
        dest: *mut B,
        src: *mut B,
        self_: *const M,
    ) -> *mut T
    where
        M: ?Sized,
        B: ?Sized,
    {
        (self.initialize_buffer_with_copy_of_buffer)(dest.cast(), src.cast(), self_.cast()).cast()
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.destroy).
    #[inline(always)]
    pub unsafe fn destroy<M, T>(&self, object: *mut T, self_: *const M)
    where
        M: ?Sized,
    {
        (self.destroy)(object.cast(), self_.cast());
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.initialize_with_copy).
    #[inline(always)]
    pub unsafe fn initialize_with_copy<M, T>(
        &self,
        dest: *mut T,
        src: *mut T,
        self_: *const M,
    ) -> *mut T
    where
        M: ?Sized,
    {
        (self.initialize_with_copy)(dest.cast(), src.cast(), self_.cast()).cast()
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.assign_with_copy).
    #[inline(always)]
    pub unsafe fn assign_with_copy<M, T>(
        &self,
        dest: *mut T,
        src: *mut T,
        self_: *const M,
    ) -> *mut T
    where
        M: ?Sized,
    {
        (self.assign_with_copy)(dest.cast(), src.cast(), self_.cast()).cast()
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.initialize_with_take).
    #[inline(always)]
    pub unsafe fn initialize_with_take<M, T>(
        &self,
        dest: *mut T,
        src: *mut T,
        self_: *const M,
    ) -> *mut T
    where
        M: ?Sized,
    {
        (self.initialize_with_take)(dest.cast(), src.cast(), self_.cast()).cast()
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.assign_with_take).
    #[inline(always)]
    pub unsafe fn assign_with_take<M, T>(
        &self,
        dest: *mut T,
        src: *mut T,
        self_: *const M,
    ) -> *mut T
    where
        M: ?Sized,
    {
        (self.assign_with_take)(dest.cast(), src.cast(), self_.cast()).cast()
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.get_enum_tag_single_payload).
    #[inline(always)]
    pub unsafe fn get_enum_tag_single_payload<M, T>(
        &self,
        enum_: *const T,
        empty_cases: c_uint,
        self_: *const M,
    ) -> c_uint
    where
        M: ?Sized,
    {
        (self.get_enum_tag_single_payload)(enum_.cast(), empty_cases, self_.cast())
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.store_enum_tag_single_payload).
    #[inline(always)]
    pub unsafe fn store_enum_tag_single_payload<M, T>(
        &self,
        enum_: *mut T,
        which_case: c_uint,
        empty_cases: c_uint,
        self_: *const M,
    ) where
        M: ?Sized,
    {
        (self.store_enum_tag_single_payload)(enum_.cast(), which_case, empty_cases, self_.cast());
    }
}

/// A value-witness table with enum entry points.
///
/// Equivalent to `EnumValueWitnessTable` in
/// [`Metadata.h`](https://github.com/apple/swift/blob/master/include/swift/Runtime/Metadata.h).
///
/// This includes all fields within `#if WANT_ENUM_VALUE_WITNESSES` in
/// [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EnumValueWitnessTable {
    /// The base value-witness table.
    pub base: ValueWitnessTable,

    /// Given a valid object of this `enum` type, extracts the tag value
    /// indicating which case of the enum is inhabited. Returned values are in
    /// the range `[0..NumElements-1]`.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// unsigned (*getEnumTag)(T *obj, M *self);
    /// ```
    pub get_enum_tag: unsafe extern "C" fn(obj: *const c_void, self_: *const c_void) -> c_uint,

    /// Given a valid object of this enum type, destructively extracts the
    /// associated payload.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// void (*destructiveProjectEnumData)(T *obj, M *self);
    /// ```
    pub destructive_project_enum_data: unsafe extern "C" fn(obj: *mut c_void, self_: *const c_void),

    /// Given an enum case tag and a valid object of case's payload type,
    /// destructively inserts the tag into the payload. The given tag value must
    /// be in the range `[-ElementsWithPayload..ElementsWithNoPayload-1]`.
    ///
    /// Equivalent to the following C prototype:
    ///
    /// ```c
    /// void (*destructiveInjectEnumTag)(T *obj, unsigned tag, M *self);
    /// ```
    pub destructive_inject_enum_tag:
        unsafe extern "C" fn(obj: *mut c_void, tag: c_uint, self_: *const c_void),
}

/// Function pointer wrapper methods.
///
/// These methods use generic parameters to:
///
/// - Make the function pointers consistent in argument and return types, making
///   it harder to misuse them.
///
/// - Simplify calling code by being able to take advantage of type inference.
impl EnumValueWitnessTable {
    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.get_enum_tag).
    #[inline(always)]
    pub unsafe fn get_enum_tag<M, T>(&self, obj: *const T, self_: *const M) -> c_uint
    where
        M: ?Sized,
    {
        (self.get_enum_tag)(obj.cast(), self_.cast())
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.destructive_project_enum_data).
    #[inline(always)]
    pub unsafe fn destructive_project_enum_data<M, T>(&self, obj: *mut T, self_: *const M)
    where
        M: ?Sized,
    {
        (self.destructive_project_enum_data)(obj.cast(), self_.cast());
    }

    /// A generic wrapper over
    /// [the corresponding function pointer](#structfield.destructive_inject_enum_tag).
    #[inline(always)]
    pub unsafe fn destructive_inject_enum_tag<M, T>(
        &self,
        obj: *mut T,
        tag: c_uint,
        self_: *const M,
    ) where
        M: ?Sized,
    {
        (self.destructive_inject_enum_tag)(obj.cast(), tag, self_.cast());
    }
}
