use std::fmt;

/// Kinds of Swift metadata records.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataKind(u32);

/// Non-type metadata kinds have this bit set.
const NON_TYPE_FLAG: u32 = 0x400;

/// Non-heap metadata kinds have this bit set.
const NON_HEAP_FLAG: u32 = 0x200;

/// Runtime-private metadata has this bit set. The compiler must not statically
/// generate metadata objects with these kinds, and external tools should not
/// rely on the stability of these values or the precise binary layout of their
/// associated data structures.
const RUNTIME_PRIVATE_FLAG: u32 = 0x100;

// Documentation taken from `TypeMetadata.rst`
impl MetadataKind {
    /// The largest possible non-isa-pointer metadata kind value.
    pub const LAST: Self = Self(0x7FF);

    /// Attempts to create new instance from `value` if valid.
    #[inline]
    pub fn new(value: u32) -> Option<Self> {
        // TODO: Make a `const fn` (https://github.com/rust-lang/rust/issues/49146)
        if value <= Self::LAST.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns new instance from `value` without checking validity.
    #[inline]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        // TODO: Add safety doc section.
        #![allow(clippy::missing_safety_doc)]

        Self(value)
    }

    /// Returns this kind's inner value.
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Returns whether `self` is type metadata.
    #[inline]
    pub const fn is_type(&self) -> bool {
        self.0 & NON_TYPE_FLAG == 0
    }

    /// Returns whether `self` is heap metadata.
    #[inline]
    pub const fn is_heap(&self) -> bool {
        self.0 & NON_HEAP_FLAG == 0
    }

    /// Returns whether `self` represents the native metadata kind for a Swift
    /// nominal type.
    #[inline]
    pub const fn is_nominal_type(&self) -> bool {
        self.is_class() | self.is_struct() | self.is_enum() | self.is_optional()
    }

    /// Returns whether `self` is runtime-private.
    ///
    /// External tools should not rely on the stability of these values or the
    /// precise binary layout of their associated data structures.
    #[inline]
    pub const fn is_runtime_private(&self) -> bool {
        self.0 & RUNTIME_PRIVATE_FLAG != 0
    }

    /// Returns `true` if `self` represents some kind of class.
    #[inline]
    pub const fn is_any_kind_of_class(&self) -> bool {
        self.is_class() | self.is_objc_class_wrapper() | self.is_foreign_class()
    }

    /// Returns `true` if `self` is for an existential type.
    #[inline]
    pub const fn is_any_existential_type(&self) -> bool {
        self.is_existential_metatype() | self.is_existential()
    }
}

macro_rules! kinds {
    ($(
        $(#[$kind_meta:meta])+
        $kind:ident = $value:expr;

        $(#[$is_kind_meta:meta])+
        $is_kind:ident;
    )+) => {
        impl fmt::Debug for MetadataKind {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Format like an `enum`.
                let known_kind = match *self {
                    $(Self::$kind => stringify!($kind),)+

                    _ => return f.debug_tuple("UNKNOWN").field(&self.0).finish(),
                };

                f.write_str(known_kind)
            }
        }

        /// Constants from [`MetadataKind.def`][file].
        ///
        /// [file]: https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataKind.def
        impl MetadataKind {
            $(
                $(#[$kind_meta])+
                pub const $kind: Self = Self($value);
            )+

            $(
                $(#[$is_kind_meta])+
                #[inline]
                pub const fn $is_kind(&self) -> bool {
                    self.0 == Self::$kind.0
                }
            )+
        }
    };
}

kinds! {
    /// A class type.
    CLASS = 0;

    /// Returns `true` if `self` is a class type.
    is_class;

    /// A struct type.
    STRUCT = NON_HEAP_FLAG;

    /// Returns `true` if `self` is a struct type.
    is_struct;

    /// An enum type.
    ENUM = 1 | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an enum type.
    is_enum;

    /// An optional type.
    OPTIONAL = 2 | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an optional type.
    is_optional;

    /// A foreign class, such as a Core Foundation class.
    FOREIGN_CLASS = 3 | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a foreign class, such as a Core Foundation
    /// class.
    is_foreign_class;

    /// A type whose value is not exposed in the metadata system.
    OPAQUE = RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a type whose value is not exposed in the
    /// metadata system.
    is_opaque;

    /// A tuple.
    TUPLE = 1 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a tuple.
    is_tuple;

    /// A monomorphic function.
    FUNCTION = 2 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a monomorphic function.
    is_function;

    /// An existential type.
    EXISTENTIAL = 3 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an existential type.
    is_existential;

    /// A metatype.
    METATYPE = 4 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is a metatype.
    is_metatype;

    /// An ObjC class wrapper.
    OBJC_CLASS_WRAPPER = 5 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an ObjC class wrapper.
    is_objc_class_wrapper;

    /// An existential metatype.
    EXISTENTIAL_METATYPE = 6 | RUNTIME_PRIVATE_FLAG | NON_HEAP_FLAG;

    /// Returns `true` if `self` is an existential metatype.
    is_existential_metatype;

    /// A heap-allocated local variable using statically-generated metadata.
    HEAP_LOCAL_VARIABLE = NON_TYPE_FLAG;

    /// Returns `true` if `self` is a heap-allocated local variable using
    /// statically-generated metadata.
    is_heap_local_variable;

    /// A heap-allocated local variable using runtime-instantiated metadata.
    HEAP_GENERIC_LOCAL_VARIABLE = NON_TYPE_FLAG | RUNTIME_PRIVATE_FLAG;

    /// Returns `true` if `self` is a heap-allocated local variable using
    /// runtime-instantiated metadata.
    is_heap_generic_local_variable;

    /// A native error object.
    ERROR_OBJECT = 1 | NON_TYPE_FLAG | RUNTIME_PRIVATE_FLAG;

    /// Returns `true` if `self` is a native error object.
    is_error_object;
}
