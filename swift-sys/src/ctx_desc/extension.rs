use crate::{ctx_desc::ContextDescriptor, ptr::RelativeDirectPointer};
use std::os::raw::c_char;

/// Descriptor for an extension context.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExtensionContextDescriptor {
    /// The base context descriptor.
    pub base: ContextDescriptor,

    /// A mangling of the `Self` type context that the extension extends.
    ///
    /// The mangled name represents the type in the generic context encoded by
    /// this descriptor. For example, a nongeneric nominal type extension will
    /// encode the nominal type name. A generic nominal type extension will
    /// encode the instance of the type with any generic arguments bound.
    ///
    /// Note that the parent of the extension will be the module context the
    /// extension is declared inside.
    pub extended_context: RelativeDirectPointer<c_char>,
}
