use std::marker::PhantomData;

/// A value about which nothing is known.
#[repr(C)]
pub struct OpaqueValue {
    // TODO: Replace with an `extern type` once stabilized.
    _private: [u8; 0],

    // !Send + !Sync
    _marker: PhantomData<*mut ()>,
}
