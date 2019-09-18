// file comparable to rust/src/liballoc/collections/btree/set.rs

use core::cmp::min;
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
        a_iter: Iter<'a, T>,
        b_iter: Iter<'a, T>,
    },
    Swivel {
        a_range: Range<'a, T>,
        a_set: &'a BTreeSet<T>,
        b_range: Range<'a, T>,
        b_set: &'a BTreeSet<T>,
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
                    a_iter: small.iter(),
                    b_iter: other.iter(),
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
                    a_iter,
                    b_iter,
                } => IntersectionInner::Stitch {
                    a_iter: a_iter.clone(),
                    b_iter: b_iter.clone(),
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
            IntersectionInner::Stitch { a_iter, b_iter } => {
                let mut a_next = a_iter.next()?;
                let mut b_next = b_iter.next()?;
                loop {
                    match Ord::cmp(a_next, b_next) {
                        Less => a_next = a_iter.next()?,
                        Greater => b_next = b_iter.next()?,
                        Equal => return Some(a_next),
                    }
                }
            }
            IntersectionInner::Swivel {
                a_range,
                a_set,
                b_range,
                b_set,
            } => {
                const NEXT_COUNT_MAX: usize = ITER_PERFORMANCE_TIPPING_SIZE_DIFF;
                let mut next_count: usize = 0;
                let mut a_next = a_range.next()?;
                let mut b_next = b_range.next()?;
                loop {
                    match Ord::cmp(a_next, b_next) {
                        Less => {
                            next_count += 1;
                            if next_count > NEXT_COUNT_MAX {
                                next_count = 0;
                                *a_range = a_set.range(b_next..);
                            }
                            a_next = a_range.next()?
                        }
                        Greater => {
                            next_count += 1;
                            if next_count > NEXT_COUNT_MAX {
                                next_count = 0;
                                *b_range = b_set.range(a_next..);
                            }
                            b_next = b_range.next()?
                        }
                        Equal => return Some(a_next),
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
            IntersectionInner::Stitch { a_iter, b_iter } => min(a_iter.len(), b_iter.len()),
            IntersectionInner::Swivel { a_set, b_set, .. } => min(a_set.len(), b_set.len()),
            IntersectionInner::Search { small_iter, .. } => small_iter.len(),
        };
        (0, Some(min_len))
    }
}

pub fn intersection_future<'a, T: Ord>(
    selve: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    (selve as &dyn JustToAppearSimilar<T>).intersection(other)
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
    a: &'a BTreeSet<T>,
    b: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    Intersection {
        inner: IntersectionInner::Stitch {
            a_iter: a.iter(),
            b_iter: b.iter(),
        },
    }
}

pub fn intersection_swivel<'a, T: Ord>(
    a: &'a BTreeSet<T>,
    b: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    Intersection {
        inner: IntersectionInner::Swivel {
            a_range: a.range(..),
            a_set: &a,
            b_range: b.range(..),
            b_set: &b,
        },
    }
}
