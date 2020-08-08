use std::fmt;

/// A context descriptor kind.
///
/// This is semantically an `enum`. However, it is defined as a `struct` in
/// order to be future-compatible.
#[repr(transparent)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ContextDescriptorKind(u8);

impl fmt::Debug for ContextDescriptorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format like an `enum`.
        let known_kind = match *self {
            Self::MODULE => "MODULE",
            Self::EXTENSION => "EXTENSION",
            Self::ANONYMOUS => "ANONYMOUS",
            Self::PROTOCOL => "PROTOCOL",
            Self::OPAQUE_TYPE => "OPAQUE_TYPE",
            Self::CLASS => "CLASS",
            Self::STRUCT => "STRUCT",
            Self::ENUM => "ENUM",
            _ => return f.debug_tuple("UNKNOWN").field(&self.0).finish(),
        };

        f.write_str(known_kind)
    }
}

impl ContextDescriptorKind {
    /// A module.
    pub const MODULE: Self = Self(0);

    /// An extension.
    pub const EXTENSION: Self = Self(1);

    /// An anonymous possibly-generic context such as a function body.
    pub const ANONYMOUS: Self = Self(2);

    /// A protocol context.
    pub const PROTOCOL: Self = Self(3);

    /// An opaque type alias.
    pub const OPAQUE_TYPE: Self = Self(4);

    /// A class.
    pub const CLASS: Self = Self::TYPE_FIRST;

    /// A struct.
    pub const STRUCT: Self = Self(Self::TYPE_FIRST.0 + 1);

    /// An enum.
    pub const ENUM: Self = Self(Self::TYPE_FIRST.0 + 2);
}

impl ContextDescriptorKind {
    /// First kind that represents a type of any sort.
    pub const TYPE_FIRST: Self = Self(16);

    /// Last kind that represents a type of any sort.
    pub const TYPE_LAST: Self = Self(31);
}

impl ContextDescriptorKind {
    /// Attempts to create new instance from `value` if valid.
    #[inline]
    pub fn new(value: u8) -> Option<Self> {
        // TODO: Make a `const fn` (https://github.com/rust-lang/rust/issues/49146)
        if value & 0x1F == value {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns new instance from `value` without checking validity.
    #[inline]
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }

    /// Returns this kind's inner value.
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Returns `true` if this is a nominal type.
    #[inline]
    pub const fn is_type(&self) -> bool {
        (self.0 >= Self::TYPE_FIRST.0) & (self.0 <= Self::TYPE_LAST.0)
    }
}
