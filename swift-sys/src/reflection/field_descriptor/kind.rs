use std::fmt;

/// A field descriptor kind.
///
/// This is semantically an `enum`. However, it is defined as a `struct` in
/// order to be future-compatible.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDescriptorKind(u16);

impl FieldDescriptorKind {
    /// Returns new instance from `value`.
    #[inline]
    pub const fn new(value: u16) -> Self {
        // Unlike other types, `new` does not return an `Option` and there is no
        // unsafe `new_unchecked` counterpart. That is because:
        //
        // - There is no guaranteed max value.
        //
        // - This type's value is stored in its own field (as opposed to inside
        //   a mask) so there are no overflow bits.

        Self(value)
    }

    /// Returns this kind's inner value.
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Returns `true` if this field descriptor is for any type of enum.
    #[inline]
    pub const fn is_any_enum(self) -> bool {
        self.is_enum() | self.is_multi_payload_enum()
    }

    /// Returns `true` if this field descriptor is for any type of class.
    #[inline]
    pub const fn is_any_class(self) -> bool {
        self.is_class() | self.is_objc_class()
    }

    /// Returns `true` if this field descriptor is for any type of protocol.
    #[inline]
    pub const fn is_any_protocol(self) -> bool {
        self.is_protocol() | self.is_class_protocol() | self.is_objc_protocol()
    }
}

macro_rules! kinds {
    ($(
        $(#[$kind_meta:meta])+
        $kind:ident = $value:expr;

        $(#[$is_kind_meta:meta])+
        $is_kind:ident;
    )+) => {
        impl fmt::Debug for FieldDescriptorKind {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Format like an `enum`.
                let known_kind = match *self {
                    $(Self::$kind => stringify!($kind),)+

                    _ => return f.debug_tuple("UNKNOWN").field(&self.0).finish(),
                };

                f.write_str(known_kind)
            }
        }

        /// Constants from [`Records.h`][file].
        ///
        /// [file]: https://github.com/apple/swift/blob/master/include/swift/Reflection/Records.h
        impl FieldDescriptorKind {
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
    /// Field descriptor for a struct type.
    STRUCT = 0;

    /// Returns `true` if this field descriptor is for a struct type.
    is_struct;

    /// Field descriptor for a class type.
    CLASS = 1;

    /// Returns `true` if this field descriptor is for a class type.
    is_class;

    /// Field descriptor for a common enum type.
    ENUM = 2;

    /// Returns `true` if this field descriptor is for a common enum type.
    is_enum;

    /// Field descriptor for a fixed-size enum type with multiple payloads.
    MULTI_PAYLOAD_ENUM = 3;

    /// Returns `true` if this field descriptor is for a fixed-size enum type
    /// with multiple payloads.
    is_multi_payload_enum;

    /// Field descriptor for an opaque Swift protocol without fields.
    PROTOCOL = 4;

    /// Returns `true` if this field descriptor is for an opaque Swift protocol
    /// without fields.
    is_protocol;

    /// Field descriptor for a class-bound Swift protocol.
    CLASS_PROTOCOL = 5;

    /// Returns `true` if this field descriptor is for a class-bound Swift
    /// protocol.
    is_class_protocol;

    /// Field descriptor for an Objective-C protocol, which may be imported or
    /// defined in Swift.
    OBJC_PROTOCOL = 6;

    /// Returns `true` if this field descriptor is for an Objective-C protocol.
    is_objc_protocol;

    /// Field descriptor for an Objective-C class, which may be imported or
    /// defined in Swift. In the former case, field type metadata is not
    /// emitted, and must be obtained from the Objective-C runtime.
    OBJC_CLASS = 7;

    /// Returns `true` if this field descriptor is for an Objective-C class.
    is_objc_class;
}
