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
    a_iter: Iter<'a, T>,
    b_iter: Iter<'a, T>,
    a_set: &'a BTreeSet<T>,
    b_set: &'a BTreeSet<T>,
}
pub struct IntersectionSwivel<'a, T: 'a> {
    a_range: Range<'a, T>,
    b_range: Range<'a, T>,
    a_set: &'a BTreeSet<T>,
    b_set: &'a BTreeSet<T>,
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
        Intersection {
            a_iter: self.iter(),
            b_iter: other.iter(),
            a_set: self,
            b_set: other,
        }
    }
}

/*
impl<T> Clone for Intersection<'_, T> {
    fn clone(&self) -> Self {
        Intersection {
            a_iter: self.a_iter.clone(),
            b_iter: self.b_iter.clone(),
            a_set: self.a_set,
            b_set: self.b_set,
        }
    }
}

*/

impl<'a, T: Ord> Intersection<'a, T> {
    fn has_tiny_remaining_a(&self) -> bool {
        self.a_iter.len() <= self.b_iter.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF
    }
    fn has_tiny_remaining_b(&self) -> bool {
        self.b_iter.len() <= self.a_iter.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF
    }
    fn use_tiny_remaining_a(&mut self, a_next: &'a T) -> Option<&'a T> {
        Intersection::use_tiny_remainder(a_next, &mut self.a_iter, self.b_set)
    }
    fn use_tiny_remaining_b(&mut self, b_next: &'a T) -> Option<&'a T> {
        Intersection::use_tiny_remainder(b_next, &mut self.b_iter, self.a_set)
    }
    fn use_tiny_remainder(
        mut next: &'a T,
        tiny_iter: &mut Iter<'a, T>,
        huge_set: &BTreeSet<T>,
    ) -> Option<&'a T> {
        loop {
            if huge_set.contains(&next) {
                return Some(next);
            }
            next = tiny_iter.next()?;
        }
    }
}

/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<'a, T: Ord> Iterator for Intersection<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let mut a_next = self.a_iter.next()?;
        let mut b_next = self.b_iter.next()?;
        loop {
            match Ord::cmp(a_next, b_next) {
                Less => {
                    if self.has_tiny_remaining_b() {
                        return self.use_tiny_remaining_b(b_next);
                    }
                    a_next = self.a_iter.next()?;
                    if self.has_tiny_remaining_a() {
                        return self.use_tiny_remaining_a(a_next);
                    }
                }
                Greater => {
                    if self.has_tiny_remaining_a() {
                        return self.use_tiny_remaining_a(a_next);
                    }
                    b_next = self.b_iter.next()?;
                    if self.has_tiny_remaining_b() {
                        return self.use_tiny_remaining_b(b_next);
                    }
                }
                Equal => return Some(a_next),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let a_len = self.a_iter.len();
        let b_len = self.b_iter.len();
        (0, Some(min(a_len, b_len)))
    }
}

impl<'a, T: Ord> Iterator for IntersectionSwivel<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        const NEXT_COUNT_MAX: usize = ITER_PERFORMANCE_TIPPING_SIZE_DIFF;
        let mut next_count: usize = 0;
        let mut a_next = self.a_range.next()?;
        let mut b_next = self.b_range.next()?;
        loop {
            match Ord::cmp(a_next, b_next) {
                Less => {
                    next_count += 1;
                    if next_count > NEXT_COUNT_MAX {
                        next_count = 0;
                        self.a_range = self.a_set.range(b_next..);
                    }
                    a_next = self.a_range.next()?
                }
                Greater => {
                    next_count += 1;
                    if next_count > NEXT_COUNT_MAX {
                        next_count = 0;
                        self.b_range = self.b_set.range(a_next..);
                    }
                    b_next = self.b_range.next()?
                }
                Equal => return Some(a_next),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let a_len = self.a_set.len();
        let b_len = self.b_set.len();
        (0, Some(min(a_len, b_len)))
    }
}

pub fn intersection_future<'a, T: Ord>(
    selve: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    (selve as &dyn JustToAppearSimilar<T>).intersection(other)
}

pub fn intersection_swivel<'a, T: Ord>(
    a: &'a BTreeSet<T>,
    b: &'a BTreeSet<T>,
) -> IntersectionSwivel<'a, T> {
    IntersectionSwivel {
        a_range: a.range(..),
        a_set: &a,
        b_range: b.range(..),
        b_set: &b,
    }
}
