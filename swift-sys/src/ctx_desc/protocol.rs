use crate::{
    ctx_desc::ContextDescriptor,
    ptr::{RelativeDirectPointer, RelativeDirectPointerNonNull},
};
use std::os::raw::c_char;

/// Context descriptor for a protocol.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ProtocolContextDescriptor {
    /// The base context descriptor.
    pub base: ContextDescriptor,

    /// The name of the protocol.
    pub name: RelativeDirectPointerNonNull<c_char>,

    /// The number of generic requirements in the requirement signature of the
    /// protocol.
    pub num_requirements_in_signature: u32,

    /// The number of requirements in the protocol.
    ///
    /// If any requirements beyond `MinimumWitnessTableSizeInWords` are present
    /// in the witness table template, they will be not be overwritten with
    /// defaults.
    pub num_requirements: u32,

    /// Associated type names, as a space-separated list in the same order as
    /// the requirements.
    pub associated_type_names: RelativeDirectPointer<c_char>,
}
