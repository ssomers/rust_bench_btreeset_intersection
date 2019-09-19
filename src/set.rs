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
        // iterate similarly sized sets jointly, spotting matches along the way
        a_iter: Iter<'a, T>,
        b_iter: Iter<'a, T>,
    },
    Search {
        // iterate a small set, look up in the large set
        small_iter: Iter<'a, T>,
        large_set: &'a BTreeSet<T>,
    },
    Answer(Option<&'a T>), // intersection found to be empty or a singleton
    Swivel {
        // iterate or jump ahead to other set's remaining lower bound
        a_range: Range<'a, T>,
        a_set: &'a BTreeSet<T>,
        b_range: Range<'a, T>,
        b_set: &'a BTreeSet<T>,
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
impl<T: Ord> JustToAppearSimilar<T> for BTreeSet<T> {
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
    /*
        #[stable(feature = "rust1", since = "1.0.0")]
    */
    fn intersection<'a>(&'a self, other: &'a BTreeSet<T>) -> Intersection<'a, T> {
        let a_len = self.len();
        let b_len = other.len();
        Intersection {
            inner: match (a_len, b_len) {
                (0, _) => IntersectionInner::Answer(None),
                (_, 0) => IntersectionInner::Answer(None),
                (1, _) => IntersectionInner::Search {
                    small_iter: self.iter(),
                    large_set: other,
                },
                (_, 1) => IntersectionInner::Search {
                    small_iter: other.iter(),
                    large_set: self,
                },
                (2..=4, 2..=4) => IntersectionInner::Stitch {
                    a_iter: self.iter(),
                    b_iter: other.iter(),
                },
                _ => {
                    let mut self_iter = self.iter();
                    let mut other_iter = other.iter();
                    let self_min = self_iter.next().unwrap();
                    let self_max = self_iter.last().unwrap();
                    let other_min = other_iter.next().unwrap();
                    let other_max = other_iter.last().unwrap();
                    match (Ord::cmp(self_max, other_min), Ord::cmp(other_max, self_min)) {
                        (Less, _) => IntersectionInner::Answer(None),
                        (_, Less) => IntersectionInner::Answer(None),
                        (Equal, _) => IntersectionInner::Answer(Some(self_max)),
                        (_, Equal) => IntersectionInner::Answer(Some(self_min)),
                        _ => {
                            if a_len <= b_len / ITER_PERFORMANCE_TIPPING_SIZE_DIFF {
                                IntersectionInner::Search {
                                    small_iter: self.iter(),
                                    large_set: other,
                                }
                            } else if b_len <= a_len / ITER_PERFORMANCE_TIPPING_SIZE_DIFF {
                                IntersectionInner::Search {
                                    small_iter: other.iter(),
                                    large_set: self,
                                }
                            } else {
                                IntersectionInner::Stitch {
                                    a_iter: self.iter(),
                                    b_iter: other.iter(),
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}

impl<T> Clone for Intersection<'_, T> {
    fn clone(&self) -> Self {
        Intersection {
            inner: match &self.inner {
                IntersectionInner::Stitch { a_iter, b_iter } => IntersectionInner::Stitch {
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
                IntersectionInner::Answer(answer) => IntersectionInner::Answer(answer.clone()),
                IntersectionInner::Swivel {
                    a_range,
                    b_range,
                    a_set,
                    b_set,
                } => IntersectionInner::Swivel {
                    a_range: a_range.clone(),
                    b_range: b_range.clone(),
                    a_set: a_set,
                    b_set: b_set,
                },
            },
        }
    }
}
/*
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
            IntersectionInner::Search {
                small_iter,
                large_set,
            } => loop {
                let small_next = small_iter.next()?;
                if large_set.contains(&small_next) {
                    return Some(small_next);
                }
            },
            IntersectionInner::Answer(answer) => answer.take(),
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
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            IntersectionInner::Stitch { a_iter, b_iter } => {
                (0, Some(min(a_iter.len(), b_iter.len())))
            }
            IntersectionInner::Search { small_iter, .. } => (0, Some(small_iter.len())),
            IntersectionInner::Answer(Some(_)) => (1, Some(1)),
            IntersectionInner::Answer(None) => (0, Some(0)),
            IntersectionInner::Swivel { a_set, b_set, .. } => {
                (0, Some(min(a_set.len(), b_set.len())))
            }
        }
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
