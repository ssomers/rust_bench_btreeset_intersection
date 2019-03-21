// file comparable to rust/src/liballoc/collections/btree/set.rs

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::btree_set::{Iter, Range};
use std::collections::BTreeSet;

pub struct Intersection<'a, T: 'a> {
    inner: IntersectionInner<'a, T>,
}
#[derive(Debug)]
enum IntersectionInner<'a, T: 'a> {
    Stitch {
        small_iter: Iter<'a, T>, // for size_hint, should be the smaller of the sets
        other_iter: Iter<'a, T>,
    },
    Spring {
        small_range: Range<'a, T>,
        small_set: &'a BTreeSet<T>,
        other_range: Range<'a, T>,
        other_set: &'a BTreeSet<T>,
    },
    Search {
        small_iter: Iter<'a, T>,
        large_set: &'a BTreeSet<T>,
    },
}

impl<'a, T: Ord> Iterator for Intersection<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match &mut self.inner {
            IntersectionInner::Stitch {
                small_iter,
                other_iter,
            } => {
                let mut small_next = small_iter.next()?;
                let mut other_next = other_iter.next()?;
                loop {
                    match Ord::cmp(small_next, other_next) {
                        Less => small_next = small_iter.next()?,
                        Greater => other_next = other_iter.next()?,
                        Equal => return Some(small_next),
                    }
                }
            }
            IntersectionInner::Spring {
                small_range,
                small_set,
                other_range,
                other_set,
            } => {
                const NEXT_COUNT_MAX: usize = 1;
                let mut next_count: usize = 0;
                let mut small_next = small_range.next()?;
                let mut other_next = other_range.next()?;
                loop {
                    match Ord::cmp(small_next, other_next) {
                        Less => {
                            next_count += 1;
                            if next_count > NEXT_COUNT_MAX {
                                next_count = 0;
                                *small_range = small_set.range(other_next..);
                            }
                            small_next = small_range.next()?;
                        }
                        Greater => {
                            next_count += 1;
                            if next_count > NEXT_COUNT_MAX {
                                next_count = 0;
                                *other_range = other_set.range(small_next..);
                            }
                            other_next = other_range.next()?;
                        }
                        Equal => return Some(small_next),
                    }
                }
            }
            IntersectionInner::Search {
                small_iter,
                large_set,
            } => loop {
                let small_next = small_iter.next()?;
                if large_set.contains(&small_next) {
                    return Some(small_next);
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper_bound = match &self.inner {
            IntersectionInner::Stitch { small_iter, .. } => small_iter.len(),
            IntersectionInner::Spring { small_set, .. } => small_set.len(),
            IntersectionInner::Search { small_iter, .. } => small_iter.len(),
        };
        (0, Some(upper_bound))
    }
}

pub fn intersection_future<'a, T: Ord>(
    selv: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    let (small, other) = if selv.len() <= other.len() {
        (selv, other)
    } else {
        (other, selv)
    };
    Intersection {
        inner:
        // The following rule:
        // - is based on the benchmarks in
        //   https://github.com/ssomers/rust_bench_btreeset_intersection;
        // - divides, rather than multiplies, to rule out overflow;
        // - avoids creating a second iterator if one of the sets is empty.
        if small.len() > other.len() / 16 {
            // Small set is not much smaller than other set, so iterate
            // both sets jointly, spotting matches along the way.
            IntersectionInner::Stitch {
                small_iter: small.iter(),
                other_iter: other.iter(),
            }
        } else {
            // Big difference in number of elements, so iterate the small set,
            // searching for matches in the large set.
            IntersectionInner::Search {
                small_iter: small.iter(),
                large_set: other,
            }
        },
    }
}

pub fn intersection_search<'a, T: Ord>(
    small: &'a BTreeSet<T>,
    large: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    debug_assert!(small.len() <= large.len());
    Intersection {
        inner: IntersectionInner::Search {
            small_iter: small.iter(),
            large_set: &large,
        },
    }
}

pub fn intersection_spring<'a, T: Ord>(
    small: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    debug_assert!(small.len() <= other.len());
    Intersection {
        inner: IntersectionInner::Spring {
            small_range: small.range(..),
            small_set: &small,
            other_range: other.range(..),
            other_set: &other,
        },
    }
}

pub fn intersection_stitch<'a, T: Ord>(
    small: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    debug_assert!(small.len() <= other.len());
    Intersection {
        inner: IntersectionInner::Stitch {
            small_iter: small.iter(),
            other_iter: other.iter(),
        },
    }
}
