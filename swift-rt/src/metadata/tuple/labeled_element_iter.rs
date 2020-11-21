use crate::metadata::{TupleMetadata, TupleMetadataElement, TupleMetadataLabelsIter};
use std::{slice, str};

/// An [`Iterator`] over labels and elements of [`TupleMetadata`].
#[derive(Debug, Clone)]
pub struct TupleMetadataLabeledElementIter<'a> {
    labels: Option<TupleMetadataLabelsIter<'a>>,
    elements: slice::Iter<'a, TupleMetadataElement<'a>>,
}

impl<'a> TupleMetadataLabeledElementIter<'a> {
    #[inline]
    pub(crate) fn new(metadata: &'a TupleMetadata) -> Self {
        Self {
            labels: metadata.labels().map(|labels| labels.into_iter()),
            elements: metadata.elements().iter(),
        }
    }
}

impl<'a> Iterator for TupleMetadataLabeledElementIter<'a> {
    type Item = (Option<&'a str>, &'a TupleMetadataElement<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.elements.next()?;

        let label = match &mut self.labels {
            None => None,
            Some(labels) => labels.next().unwrap_or(
                // Silently fail since there's not much we can do here.
                None,
            ),
        };

        Some((label, element))
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

impl ExactSizeIterator for TupleMetadataLabeledElementIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl std::iter::FusedIterator for TupleMetadataLabeledElementIter<'_> {}
