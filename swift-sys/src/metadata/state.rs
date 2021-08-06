use std::fmt;

/// The public state of a metadata.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataState(usize);

impl MetadataState {
    /// Attempts to create new instance from `value` if valid.
    #[inline]
    pub fn new(value: usize) -> Option<Self> {
        // TODO: Make a `const fn` (https://github.com/rust-lang/rust/issues/49146)
        if value <= 0xFF {
            Some(unsafe { Self::new_unchecked(value) })
        } else {
            None
        }
    }

    /// Creates a new instance from `value` without checking validity.
    #[inline]
    pub const unsafe fn new_unchecked(value: usize) -> Self {
        // TODO: Add safety doc section.
        #![allow(clippy::missing_safety_doc)]

        Self(value)
    }

    /// Returns this state's inner value.
    #[inline]
    pub const fn value(self) -> usize {
        self.0
    }
}

macro_rules! states {
    ($(
        $(#[$state_meta:meta])+
        $state:ident = $value:expr;

        $(#[$is_state_meta:meta])+
        $is_state:ident;
    )+) => {
        impl fmt::Debug for MetadataState {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Format like an `enum`.
                let known_state = match *self {
                    $(Self::$state => stringify!($state),)+

                    _ => return f.debug_tuple("UNKNOWN").field(&self.0).finish(),
                };

                f.write_str(known_state)
            }
        }

        impl MetadataState {
            $(
                $(#[$state_meta])+
                pub const $state: Self = Self($value);
            )+

            $(
                $(#[$is_state_meta])+
                #[inline]
                pub const fn $is_state(&self) -> bool {
                    self.0 == Self::$state.0
                }
            )+
        }
    };
}

// Documentation adapted from `MetadataValues.h`.
states! {
    /// The metadata is fully complete.
    ///
    /// By definition, this is the end-state of all metadata. Generally,
    /// metadata is expected to be complete before it can be passed to arbitrary
    /// code, e.g. as a generic argument to a function or as a metatype value.
    ///
    /// In addition to the requirements of `NON_TRANSITIVE_COMPLETE`, certain
    /// transitive completeness guarantees must hold. Most importantly, complete
    /// nominal type metadata transitively guarantee the completion of their
    /// stored generic type arguments and superclass metadata.
    COMPLETE = 0x00;

    /// Returns `true` if the metadata is fully complete.
    is_complete;

    /// The metadata is fully complete except for any transitive completeness
    /// guarantees.
    ///
    /// In addition to the requirements of `LAYOUT_COMPLETE`, metadata in this
    /// state must be prepared for all basic type operations. This includes:
    ///
    /// - Any sort of internal layout necessary to allocate and work with
    ///   concrete values of the type, such as the instance layout of a class;
    ///
    /// - Any sort of external dynamic registration that might be required for
    ///   the type, such as the realization of a class by the Objective-C
    ///   runtime; and
    ///
    /// - The initialization of any other information kept in the metadata
    ///   object, such as a class's v-table.
    NON_TRANSITIVE_COMPLETE = 0x01;

    /// Returns `true` if the metadata is fully complete except for any
    /// transitive completeness guarantees.
    is_non_transitive_complete;

    /// The metadata is ready for the layout of other types that store values of
    /// this type.
    ///
    /// In addition to the requirements of `ABSTRACT`, metadata in this state
    /// must have a valid value witness table, meaning that its size, alignment,
    /// and basic type properties (such as POD-ness) have been computed.
    LAYOUT_COMPLETE = 0x3F;

    /// Returns `true` if the metadata is ready for the layout of other types
    /// that store values of this type.
    is_layout_complete;

    /// The metadata has its basic identity established. It is possible to
    /// determine what formal type it corresponds to. Among other things, it is
    /// possible to use the runtime mangling facilities with the type.
    ///
    /// For example, a metadata for a generic struct has a metadata kind, a type
    /// descriptor, and all of its type arguments. However, it does not
    /// necessarily have a meaningful value-witness table.
    ///
    /// References to other types that are not part of the type's basic identity
    /// may not yet have been established. Most crucially, this includes the
    /// superclass pointer.
    ABSTRACT = 0xFF;

    /// Returns `true` if the metadata has its basic identity established.
    is_abstract;
}
