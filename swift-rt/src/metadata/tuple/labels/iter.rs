use crate::metadata::TupleMetadataLabels;
use std::{slice, str};

/// An [`Iterator`] over [`TupleMetadataLabels`].
#[derive(Clone, Debug)]
pub struct TupleMetadataLabelsIter<'a> {
    labels: &'a TupleMetadataLabels,
}

impl<'a> TupleMetadataLabelsIter<'a> {
    #[inline]
    pub(crate) fn new(labels: &'a TupleMetadataLabels) -> Self {
        Self { labels }
    }
}

impl<'a> Iterator for TupleMetadataLabelsIter<'a> {
    type Item = Option<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.labels.as_ptr().cast::<u8>();
        let mut len = 0;

        let new_start = loop {
            let position = unsafe { start.add(len) };
            let current = unsafe { *position };

            match current {
                // The string is null-terminated.
                0 => return None,

                // Labels are space-terminated.
                b' ' => break unsafe { position.add(1) },

                // Otherwise, continue onto the next character.
                _ => len += 1,
            };
        };

        // SAFETY: `new_start` is not null and still points to valid UTF-8.
        self.labels = unsafe { TupleMetadataLabels::new_unchecked(new_start.cast()) };

        if len == 0 {
            Some(None)
        } else {
            // SAFETY: The label is UTF-8 and valid up to `len`.
            Some(Some(unsafe {
                str::from_utf8_unchecked(slice::from_raw_parts(start, len))
            }))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl ExactSizeIterator for TupleMetadataLabelsIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        let mut position = self.labels.as_ptr().cast::<u8>();
        let mut len = 0;

        unsafe {
            // The string is null-terminated.
            while *position != 0 {
                // Labels are space-terminated.
                len += (*position == b' ') as usize;

                position = position.add(1);
            }
        }

        len
    }
}

impl std::iter::FusedIterator for TupleMetadataLabelsIter<'_> {}
