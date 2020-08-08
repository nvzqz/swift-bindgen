use crate::metadata::MetadataState;

/// Kinds of requests for metadata.
///
/// The [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
/// value is a blocking request for complete metadata.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataRequest {
    value: usize,
}

const STATE_BIT: usize = 0;
const STATE_MASK: usize = 0xFF;

const NON_BLOCKING_BIT: usize = 8;

impl MetadataRequest {
    /// Creates a request in the given state and blocking type.
    #[inline]
    pub const fn new(state: MetadataState, is_non_blocking: bool) -> Self {
        Self {
            value: (state.value() << STATE_BIT) | ((is_non_blocking as usize) << NON_BLOCKING_BIT),
        }
    }

    /// Creates a request that will block until the runtime is able to produce
    /// metadata in the given state.
    #[inline]
    pub const fn blocking(state: MetadataState) -> Self {
        Self::new(state, false)
    }

    /// Creates a request that will return "immediately", producing an abstract
    /// metadata and a flag saying that the operation failed.
    #[inline]
    pub const fn non_blocking(state: MetadataState) -> Self {
        Self::new(state, true)
    }

    /// The requested state of the metadata.
    #[inline]
    pub const fn state(&self) -> MetadataState {
        unsafe { MetadataState::new_unchecked((self.value >> STATE_BIT) & STATE_MASK) }
    }

    /// Returns `true` if the request will not return until the runtime is able
    /// to produce metadata with the given kind.
    ///
    /// A non-blocking request will return "immediately", producing an abstract
    /// metadata and a flag saying that the operation failed.
    #[inline]
    pub const fn is_blocking(&self) -> bool {
        (self.value >> NON_BLOCKING_BIT) & 1 == 0
    }
}
