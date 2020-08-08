use crate::metadata::MetadataKind;
use std::{ffi::c_void, fmt, ptr};

/// Type metadata.
#[repr(C)]
pub struct Metadata {
    // Of type `StoredPointer`, which is a `u32` or `u64` based on target arch.
    kind: usize,
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Dynamically format as the appropriate type of metadata.
        f.debug_struct("Metadata").finish()
    }
}

impl Metadata {
    /// Returns the kind of this metadata.
    #[inline]
    pub fn kind(&self) -> MetadataKind {
        if self.kind <= MetadataKind::LAST.value() as usize {
            unsafe { MetadataKind::new_unchecked(self.kind as u32) }
        } else {
            MetadataKind::CLASS
        }
    }

    /// Returns the stored class isa pointer, or null if there isn't one.
    #[inline]
    pub fn isa_ptr(&self) -> *const c_void {
        if self.kind > MetadataKind::LAST.value() as usize {
            self.kind as *const c_void
        } else {
            ptr::null()
        }
    }
}
