use crate::metadata::{MetadataRequest, MetadataResponse};
use std::fmt;

/// Pointer to a metadata access function.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataAccessFunction(
    // TODO: Make this be `extern "Swift"`.
    unsafe extern "C" fn(MetadataRequest) -> MetadataResponse,
);

impl fmt::Debug for MetadataAccessFunction {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Pointer for MetadataAccessFunction {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl MetadataAccessFunction {
    // TODO: Implement methods to call `function`.
}
