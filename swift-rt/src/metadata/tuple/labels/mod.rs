use std::{ffi::CStr, fmt, os::raw::c_char, slice, str};

mod iter;
mod tests;

pub use iter::*;

/// Labels of [`TupleMetadata`](super::TupleMetadata).
///
/// This type is implemented as a C string, so a length calculation is not
/// performed until it is iterated over, or the underlying
#[repr(C)]
pub struct TupleMetadataLabels {
    // TODO: Use `extern type` once stabilized.
    // See https://github.com/rust-lang/rust/issues/43467.
    data: [c_char; 0],
}

impl fmt::Debug for TupleMetadataLabels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl PartialEq for TupleMetadataLabels {
    fn eq(&self, other: &Self) -> bool {
        let mut this = self.as_ptr();
        let mut other = other.as_ptr();

        if this == other {
            return true;
        }

        loop {
            unsafe {
                match (*this, *other) {
                    (0, 0) => return true,

                    (a, b) if a == b => {
                        this = this.add(1);
                        other = other.add(1);
                    }

                    // This case handles either being 0.
                    _ => return false,
                }
            }
        }
    }
}

impl Eq for TupleMetadataLabels {}

impl<'a> IntoIterator for &'a TupleMetadataLabels {
    type Item = Option<&'a str>;
    type IntoIter = TupleMetadataLabelsIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        TupleMetadataLabelsIter::new(self)
    }
}

impl TupleMetadataLabels {
    #[inline]
    pub(crate) unsafe fn new<'a>(start: *const c_char) -> Option<&'a Self> {
        start.cast::<Self>().as_ref()
    }

    #[inline]
    pub(crate) unsafe fn new_unchecked<'a>(start: *const c_char) -> &'a Self {
        &*start.cast::<Self>()
    }

    #[inline]
    fn str_len(&self) -> usize {
        let start = self.as_ptr();
        let mut len = 0;

        while unsafe { *start.add(len) } != 0 {
            len += 1;
        }

        len
    }

    /// Returns the pointer to the inner C string used to represent the labels.
    #[inline]
    pub fn as_ptr(&self) -> *const c_char {
        self.data.as_ptr()
    }

    /// Returns the C string used to represent the labels.
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        // SAFETY: The pointer always refers to a valid C string.
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }

    /// Returns the UTF-8 string used to represent the labels.
    ///
    /// A length calculation is performed whenever this method is called.
    #[inline]
    pub fn to_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                self.as_ptr().cast::<u8>(),
                self.str_len(),
            ))
        }
    }

    /// Returns the UTF-8 string used to represent the labels, containing the
    /// trailing 0 byte.
    ///
    /// A length calculation is performed whenever this method is called.
    #[inline]
    pub fn to_str_with_nul(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                self.as_ptr().cast::<u8>(),
                self.str_len() + 1,
            ))
        }
    }
}
