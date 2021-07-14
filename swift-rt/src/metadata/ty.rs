use crate::metadata::{Metadata, StructMetadata, TupleMetadata};

// Used for simplifying doc comments.
#[allow(unused_imports)]
use swift_sys::metadata::ValueWitnessTable;

/// A type whose runtime type information can be obtained without an existing
/// instance.
///
/// Convenience metadata property methods are provided to enable avoiding
/// runtime calls for properties known at compile-time in Rust. These could be
/// as cheap as having values be emitted directly in the Rust binary.
pub trait Type {
    /// The specific runtime metadata type for `Self`.
    type Metadata: AsRef<Metadata>;

    /// A shorthand for [`ValueWitnessTable::is_pod`] that enables compile-time
    /// optimization if a constant value is provided.
    fn is_pod() -> bool {
        Self::get_metadata().as_ref().value_witnesses().is_pod()
    }

    /// A shorthand for [`ValueWitnessTable::is_bitwise_takable`] that enables
    /// compile-time optimization if a constant value is provided.
    fn is_bitwise_takable() -> bool {
        Self::get_metadata()
            .as_ref()
            .value_witnesses()
            .is_bitwise_takable()
    }

    // Note that `'a` is used in order to make the above convenience functions
    // work without `Metadata: 'static`.

    /// Requests the runtime metadata for this static type, potentially blocking
    /// until the type is completely defined by the runtime.
    ///
    /// For runtime types known at compile-time this should simply reference its
    /// symbol directly, such as `$sSbN` for `bool`.
    fn get_metadata<'a>() -> &'a Self::Metadata;

    /// Requests the runtime metadata for this static type.
    ///
    /// If `blocking` is `true`, this is the same as calling
    /// [`Self::get_metadata`].
    fn get_metadata_blocking<'a>(blocking: bool) -> Option<&'a Self::Metadata>;
}

// TODO: Use `swift_getTupleTypeMetadata2` for 2-ary tuples.
// TODO: Use `swift_getTupleTypeMetadata3` for 3-ary tuples.
// TODO: Use `swift_getTupleTypeMetadata` for n-ary tuples.

macro_rules! imp_static {
    ($($ty:ty => $metadata_ty:ty, $sym:expr;)+) => {
        $(
            impl Type for $ty {
                type Metadata = $metadata_ty;

                #[inline]
                fn is_pod() -> bool {
                    true
                }

                #[inline]
                fn is_bitwise_takable() -> bool {
                    true
                }

                #[inline]
                fn get_metadata<'a>() -> &'a $metadata_ty {
                    extern "C" {
                        #[link_name = $sym]
                        static METADATA: $metadata_ty;
                    }
                    unsafe { &METADATA }
                }

                #[inline]
                fn get_metadata_blocking<'a>(_blocking: bool) -> Option<&'a $metadata_ty> {
                    Some(Self::get_metadata())
                }
            }
        )+
    };
}

impl Type for () {
    type Metadata = TupleMetadata;

    #[inline]
    fn is_pod() -> bool {
        true
    }

    #[inline]
    fn is_bitwise_takable() -> bool {
        true
    }

    #[inline]
    fn get_metadata<'a>() -> &'a TupleMetadata {
        // TODO: Expose full metadata type.
        #[repr(C)]
        struct FullMetadata {
            header: usize,
            metadata: TupleMetadata,
        }

        extern "C" {
            #[link_name = "$sytN"]
            static METADATA: FullMetadata;
        }
        unsafe { &METADATA.metadata }
    }

    #[inline]
    fn get_metadata_blocking<'a>(_blocking: bool) -> Option<&'a TupleMetadata> {
        Some(Self::get_metadata())
    }
}

imp_static! {
    bool /* Bool */ => StructMetadata, "$sSbN";

    f32 /* Float  / "Float32" */ => StructMetadata, "$sSfN";
    f64 /* Double / "Float64" */ => StructMetadata, "$sSdN";

    isize  /*   Int   */ => StructMetadata, "$sSiN";
    usize  /*  UInt   */ => StructMetadata, "$sSuN";
    i8     /*   Int8  */ => StructMetadata, "$ss4Int8VN";
    u8     /*  UInt8  */ => StructMetadata, "$ss5UInt8VN";
    i16    /*   Int16 */ => StructMetadata, "$ss5Int16VN";
    u16    /*  UInt16 */ => StructMetadata, "$ss6UInt16VN";
    i32    /*   Int32 */ => StructMetadata, "$ss5Int32VN";
    u32    /*  UInt32 */ => StructMetadata, "$ss6UInt32VN";
    i64    /*   Int64 */ => StructMetadata, "$ss5Int64VN";
    u64    /*  UInt64 */ => StructMetadata, "$ss6UInt64VN";

    char /* Unicode.Scalar */ => StructMetadata, "$ss7UnicodeO6ScalarVN";
}

#[cfg(test)]
mod tests {
    use super::*;
    use swift_sys::metadata::MetadataKind;

    #[test]
    fn tuple() {
        let metadata: &Metadata = <()>::get_metadata().as_ref();
        assert_eq!(metadata.kind(), MetadataKind::TUPLE);
    }
}
