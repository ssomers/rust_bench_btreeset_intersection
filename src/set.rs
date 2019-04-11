// file comparable to rust/src/liballoc/collections/btree/set.rs

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::btree_set::{Iter, Range};
use std::collections::BTreeSet;

/// A lazy iterator producing elements in the intersection of `BTreeSet`s.
///
/// This `struct` is created by the [`intersection`] method on [`BTreeSet`].
/// See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`intersection`]: struct.BTreeSet.html#method.intersection
pub struct Intersection<'a, T: 'a> {
    inner: IntersectionInner<'a, T>,
}
enum IntersectionInner<'a, T: 'a> {
    Stitch {
        small_iter: Iter<'a, T>, // for size_hint, should be the smaller of the sets
        other_iter: Iter<'a, T>,
    },
    Swivel {
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

// This constant is used by functions that compare two sets.
// It estimates the relative size at which searching performs better
// than iterating, based on the benchmarks in
// https://github.com/ssomers/rust_bench_btreeset_intersection;
// It's used to divide rather than multiply sizes, to rule out overflow,
// and it's a power of two to make that division cheap.
const ITER_PERFORMANCE_TIPPING_SIZE_DIFF: usize = 16;

trait JustToAppearSimilar<T> {
    fn intersection<'a>(&'a self, other: &'a BTreeSet<T>) -> Intersection<'a, T>;
}
impl<T> JustToAppearSimilar<T> for BTreeSet<T> {
    /// Visits the values representing the intersection,
    /// i.e., the values that are both in `self` and `other`,
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut a = BTreeSet::new();
    /// a.insert(1);
    /// a.insert(2);
    ///
    /// let mut b = BTreeSet::new();
    /// b.insert(2);
    /// b.insert(3);
    ///
    /// let intersection: Vec<_> = a.intersection(&b).cloned().collect();
    /// assert_eq!(intersection, [2]);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    fn intersection<'a>(&'a self, other: &'a BTreeSet<T>) -> Intersection<'a, T> {
        let (small, other) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        if small.len() > other.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF {
            // Small set is not much smaller than other set.
            // Iterate both sets jointly, spotting matches along the way.
            Intersection {
                inner: IntersectionInner::Stitch {
                    small_iter: small.iter(),
                    other_iter: other.iter(),
                },
            }
        } else {
            // Big difference in number of elements, or both sets are empty.
            // Iterate the small set, searching for matches in the large set.
            Intersection {
                inner: IntersectionInner::Search {
                    small_iter: small.iter(),
                    large_set: other,
                },
            }
        }
    }
}

/*
impl<T> Clone for Intersection<'_, T> {
    fn clone(&self) -> Self {
        Intersection {
            inner: match &self.inner {
                IntersectionInner::Stitch {
                    small_iter,
                    other_iter,
                } => IntersectionInner::Stitch {
                    small_iter: small_iter.clone(),
                    other_iter: other_iter.clone(),
                },
                IntersectionInner::Search {
                    small_iter,
                    large_set,
                } => IntersectionInner::Search {
                    small_iter: small_iter.clone(),
                    large_set,
                },
            },
        }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
*/
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
            IntersectionInner::Swivel {
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
        let min_len = match &self.inner {
            IntersectionInner::Stitch { small_iter, .. } => small_iter.len(),
            IntersectionInner::Swivel { small_set, .. } => small_set.len(),
            IntersectionInner::Search { small_iter, .. } => small_iter.len(),
        };
        (0, Some(min_len))
    }
}

pub fn intersection_future<'a, T: Ord>(
    selve: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    (selve as &JustToAppearSimilar<T>).intersection(other)
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

pub fn intersection_swivel<'a, T: Ord>(
    small: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    debug_assert!(small.len() <= other.len());
    Intersection {
        inner: IntersectionInner::Swivel {
            small_range: small.range(..),
            small_set: &small,
            other_range: other.range(..),
            other_set: &other,
        },
    }
}
