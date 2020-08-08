//! Swift type reflection.

mod field_descriptor;
mod field_record;

pub use field_descriptor::*;
pub use field_record::*;

#[doc(no_inline)]
pub use swift_sys::reflection::{FieldDescriptorKind, FieldRecordFlags};
