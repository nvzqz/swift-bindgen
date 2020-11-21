#![cfg(test)]

use super::*;
use std::iter;

const MAX_LABELS: usize = 10;

#[test]
fn empty() {
    let expected = vec![];

    let labels = unsafe { TupleMetadataLabels::new(b"\0".as_ptr().cast()) }.unwrap();
    let labels = labels.into_iter().collect::<Vec<_>>();

    assert_eq!(labels, expected);
}

#[test]
fn all_none() {
    for n in 1..=MAX_LABELS {
        let expected = vec![None; n];

        let mut labels = vec![b' '; n];
        labels.push(0);

        let labels = unsafe { TupleMetadataLabels::new(labels.as_ptr().cast()) }.unwrap();
        let labels = labels.into_iter().collect::<Vec<_>>();

        assert_eq!(labels, expected);
    }
}

#[test]
fn all_some() {
    let label = "ábcdë";

    for n in 1..=MAX_LABELS {
        let expected = vec![Some(label); n];

        let mut labels = Vec::<u8>::new();
        labels.extend(
            iter::repeat(format!("{} ", label).as_bytes())
                .take(n)
                .flatten(),
        );
        labels.push(0);

        let labels = unsafe { TupleMetadataLabels::new(labels.as_ptr().cast()) }.unwrap();
        let labels = labels.into_iter().collect::<Vec<_>>();

        assert_eq!(labels, expected);
    }
}
