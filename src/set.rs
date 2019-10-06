// file comparable to rust/src/liballoc/collections/btree/set.rs
use core::cmp::Ordering::{Equal, Greater, Less};
use core::cmp::{max, min};
use core::fmt::{self};
use core::iter::FusedIterator;
use std::collections::btree_set::{Iter, Range};
use std::collections::BTreeSet;

/*
// This is pretty much entirely stolen from TreeSet, since BTreeMap has an identical interface
// to TreeMap

use core::borrow::Borrow;
use core::cmp::Ordering::{Less, Greater, Equal};
use core::cmp::{max, min};
use core::fmt::{self, Debug};
use core::iter::{FromIterator, FusedIterator};
use core::ops::{BitOr, BitAnd, BitXor, Sub, RangeBounds};

use crate::collections::btree_map::{self, BTreeMap, Keys};
use super::Recover;

// FIXME(conventions): implement bounded iterators

/// A set based on a B-Tree.
///
/// See [`BTreeMap`]'s documentation for a detailed discussion of this collection's performance
/// benefits and drawbacks.
///
/// It is a logic error for an item to be modified in such a way that the item's ordering relative
/// to any other item, as determined by the [`Ord`] trait, changes while it is in the set. This is
/// normally only possible through [`Cell`], [`RefCell`], global state, I/O, or unsafe code.
///
/// [`BTreeMap`]: struct.BTreeMap.html
/// [`Ord`]: ../../std/cmp/trait.Ord.html
/// [`Cell`]: ../../std/cell/struct.Cell.html
/// [`RefCell`]: ../../std/cell/struct.RefCell.html
///
/// # Examples
///
/// ```
/// use std::collections::BTreeSet;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BTreeSet<&str>` in this example).
/// let mut books = BTreeSet::new();
///
/// // Add some books.
/// books.insert("A Dance With Dragons");
/// books.insert("To Kill a Mockingbird");
/// books.insert("The Odyssey");
/// books.insert("The Great Gatsby");
///
/// // Check for a specific one.
/// if !books.contains("The Winds of Winter") {
///     println!("We have {} books, but The Winds of Winter ain't one.",
///              books.len());
/// }
///
/// // Remove a book.
/// books.remove("The Odyssey");
///
/// // Iterate over everything.
/// for book in &books {
///     println!("{}", book);
/// }
/// ```
#[derive(Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct BTreeSet<T> {
    map: BTreeMap<T, ()>,
}

/// An iterator over the items of a `BTreeSet`.
///
/// This `struct` is created by the [`iter`] method on [`BTreeSet`].
/// See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`iter`]: struct.BTreeSet.html#method.iter
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Iter<'a, T: 'a> {
    iter: Keys<'a, T, ()>,
}

#[stable(feature = "collection_debug", since = "1.17.0")]
impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter")
         .field(&self.iter.clone())
         .finish()
    }
}

/// An owning iterator over the items of a `BTreeSet`.
///
/// This `struct` is created by the [`into_iter`] method on [`BTreeSet`][`BTreeSet`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`into_iter`]: struct.BTreeSet.html#method.into_iter
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Debug)]
pub struct IntoIter<T> {
    iter: btree_map::IntoIter<T, ()>,
}

/// An iterator over a sub-range of items in a `BTreeSet`.
///
/// This `struct` is created by the [`range`] method on [`BTreeSet`].
/// See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`range`]: struct.BTreeSet.html#method.range
#[derive(Debug)]
#[stable(feature = "btree_range", since = "1.17.0")]
pub struct Range<'a, T: 'a> {
    iter: btree_map::Range<'a, T, ()>,
}
*/

/// Faster and smaller aLternative to Peekable, if copying items is cheap
/// and if there is no penalty for peeking early.
/// It always has one item read ahead.
#[derive(Clone)]
struct Peeking<I>
where
    I: Iterator,
    I::Item: Copy,
{
    head: Option<I::Item>,
    tail: I,
}
impl<I> Peeking<I>
where
    I: ExactSizeIterator + FusedIterator + Clone,
    I::Item: Copy,
{
    fn new(mut iter: I) -> Self {
        let head = iter.next();
        Peeking { head, tail: iter }
    }

    fn peek(&self) -> Option<I::Item> {
        self.head
    }

    fn next(&mut self) -> Option<I::Item> {
        let next = self.head;
        self.head = self.tail.next();
        next
    }

    fn len(&self) -> usize {
        match self.head {
            None => 0,
            Some(_) => 1 + self.tail.len(),
        }
    }
}

/// Core of SymmetricDifference and Union.
/// More efficient than btree.map.MergeIter and has next()
/// reporting how many sets contained the element (if any).
struct MergeIter<I>
where
    I: Iterator,
    I::Item: Copy,
{
    a: Peeking<I>,
    b: Peeking<I>,
}
impl<I> Clone for MergeIter<I>
where
    I: Clone + Iterator,
    I::Item: Copy,
{
    fn clone(&self) -> Self {
        MergeIter {
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}
impl<I> MergeIter<I>
where
    I: Clone + ExactSizeIterator + FusedIterator,
    I::Item: Copy + Ord,
{
    fn new(a: I, b: I) -> Self {
        MergeIter {
            a: Peeking::new(a),
            b: Peeking::new(b),
        }
    }

    fn next(&mut self) -> Option<I::Item> {
        self.next_counted().0
    }

    fn next_counted(&mut self) -> (Option<I::Item>, usize) {
        let ord = match (self.a.peek(), self.b.peek()) {
            (None, None) => return (None, 0),
            (_, None) => Less,
            (None, _) => Greater,
            (Some(a), Some(b)) => a.cmp(&b),
        };
        match ord {
            Less => (self.a.next(), 1),
            Greater => (self.b.next(), 1),
            Equal => {
                self.b.next();
                (self.a.next(), 2)
            }
        }
    }
}

/// A lazy iterator producing elements in the difference of `BTreeSet`s.
///
/// This `struct` is created by the [`difference`] method on [`BTreeSet`].
/// See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`difference`]: struct.BTreeSet.html#method.difference
/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
pub struct Difference<'a, T: 'a> {
    inner: DifferenceInner<'a, T>,
}
enum DifferenceInner<'a, T: 'a> {
    Stitch {
        // iterate all of self and some of other, spotting matches along the way
        self_iter: Iter<'a, T>,
        other_iter: Peeking<Iter<'a, T>>,
    },
    Search {
        // iterate a small set, look up in the large set
        self_iter: Iter<'a, T>,
        other_set: &'a BTreeSet<T>,
    },
    Iterate(Iter<'a, T>), // simply stream self's elements
}

/*
#[stable(feature = "collection_debug", since = "1.17.0")]
*/
impl<T: fmt::Debug> fmt::Debug for Difference<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            DifferenceInner::Stitch {
                self_iter,
                other_iter,
            } => f
                .debug_tuple("Difference")
                .field(&self_iter)
                .field(&other_iter.head)
                .field(&other_iter.tail)
                .finish(),
            DifferenceInner::Search {
                self_iter,
                other_set: _,
            } => f.debug_tuple("Difference").field(&self_iter).finish(),
            DifferenceInner::Iterate(iter) => f.debug_tuple("Difference").field(&iter).finish(),
        }
    }
}

/// A lazy iterator producing elements in the symmetric difference of `BTreeSet`s.
///
/// This `struct` is created by the [`symmetric_difference`] method on
/// [`BTreeSet`]. See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`symmetric_difference`]: struct.BTreeSet.html#method.symmetric_difference
/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
pub struct SymmetricDifference<'a, T: 'a>(MergeIter<Iter<'a, T>>);

/*
#[stable(feature = "collection_debug", since = "1.17.0")]
*/
impl<T: fmt::Debug> fmt::Debug for SymmetricDifference<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SymmetricDifference")
            .field(&self.0.a.head)
            .field(&self.0.a.tail)
            .field(&self.0.b.head)
            .field(&self.0.b.tail)
            .finish()
    }
}

/// A lazy iterator producing elements in the intersection of `BTreeSet`s.
///
/// This `struct` is created by the [`intersection`] method on [`BTreeSet`].
/// See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`intersection`]: struct.BTreeSet.html#method.intersection
/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
pub struct Intersection<'a, T: 'a> {
    inner: IntersectionInner<'a, T>,
}
enum IntersectionInner<'a, T: 'a> {
    Stitch {
        // iterate similarly sized sets jointly, spotting matches along the way
        a: Iter<'a, T>,
        b: Iter<'a, T>,
    },
    Search {
        // iterate a small set, look up in the large set
        small_iter: Iter<'a, T>,
        large_set: &'a BTreeSet<T>,
    },
    Answer(Option<&'a T>), // return a specific value or emptiness
}

/*
#[stable(feature = "collection_debug", since = "1.17.0")]
*/
impl<T: fmt::Debug> fmt::Debug for Intersection<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            IntersectionInner::Stitch {
                a,
                b,
            } => f
                .debug_tuple("Intersection")
                .field(&a)
                .field(&b)
                .finish(),
            IntersectionInner::Search {
                small_iter,
                large_set: _,
            } => f.debug_tuple("Intersection").field(&small_iter).finish(),
            IntersectionInner::Answer(answer) => {
                f.debug_tuple("Intersection").field(&answer).finish()
            }
        }
    }
}

/// A lazy iterator producing elements in the union of `BTreeSet`s.
///
/// This `struct` is created by the [`union`] method on [`BTreeSet`].
/// See its documentation for more.
///
/// [`BTreeSet`]: struct.BTreeSet.html
/// [`union`]: struct.BTreeSet.html#method.union
/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
pub struct Union<'a, T: 'a>(MergeIter<Iter<'a, T>>);

/*
#[stable(feature = "collection_debug", since = "1.17.0")]
*/
impl<T: fmt::Debug> fmt::Debug for Union<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Union")
            .field(&self.0.a.head)
            .field(&self.0.a.tail)
            .field(&self.0.b.head)
            .field(&self.0.b.tail)
            .finish()
    }
}

// This constant is used by functions that compare two sets.
// It estimates the relative size at which searching performs better
// than iterating, based on the benchmarks in
// https://github.com/ssomers/rust_bench_btreeset_intersection;
// It's used to divide rather than multiply sizes, to rule out overflow,
// and it's a power of two to make that division cheap.
const ITER_PERFORMANCE_TIPPING_SIZE_DIFF: usize = 16;

/*
impl<T: Ord> BTreeSet<T> {
    /// Makes a new `BTreeSet` with a reasonable choice of B.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use std::collections::BTreeSet;
    ///
    /// let mut set: BTreeSet<i32> = BTreeSet::new();
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new() -> BTreeSet<T> {
        BTreeSet { map: BTreeMap::new() }
    }

    /// Constructs a double-ended iterator over a sub-range of elements in the set.
    /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
    /// yield elements from min (inclusive) to max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
    /// `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
    /// range from 4 to 10.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use std::ops::Bound::Included;
    ///
    /// let mut set = BTreeSet::new();
    /// set.insert(3);
    /// set.insert(5);
    /// set.insert(8);
    /// for &elem in set.range((Included(&4), Included(&8))) {
    ///     println!("{}", elem);
    /// }
    /// assert_eq!(Some(&5), set.range(4..).next());
    /// ```
    #[stable(feature = "btree_range", since = "1.17.0")]
    pub fn range<K: ?Sized, R>(&self, range: R) -> Range<'_, T>
        where K: Ord, T: Borrow<K>, R: RangeBounds<K>
    {
        Range { iter: self.map.range(range) }
    }

    /// Visits the values representing the difference,
    /// i.e., the values that are in `self` but not in `other`,
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
    /// let diff: Vec<_> = a.difference(&b).cloned().collect();
    /// assert_eq!(diff, [1]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn difference<'a>(&'a self, other: &'a BTreeSet<T>) -> Difference<'a, T> {
    */
trait JustToIndentAsMuch<T> {
    fn difference<'a>(&'a self, other: &'a BTreeSet<T>) -> Difference<'a, T>;
    fn symmetric_difference<'a>(&'a self, other: &'a BTreeSet<T>) -> SymmetricDifference<'a, T>;
    fn intersection<'a>(&'a self, other: &'a BTreeSet<T>) -> Intersection<'a, T>;
    fn union<'a>(&'a self, other: &'a BTreeSet<T>) -> Union<'a, T>;
    fn is_subset(&self, other: &BTreeSet<T>) -> bool;
}
impl<T: Ord> JustToIndentAsMuch<T> for BTreeSet<T> {
    fn difference<'a>(&'a self, other: &'a BTreeSet<T>) -> Difference<'a, T> {
        let (self_min, self_max) = if let (Some(self_min), Some(self_max)) =
            (self.iter().next(), self.iter().next_back())
        {
            (self_min, self_max)
        } else {
            return Difference {
                inner: DifferenceInner::Iterate(self.iter()),
            };
        };
        let (other_min, other_max) = if let (Some(other_min), Some(other_max)) =
            (other.iter().next(), other.iter().next_back())
        {
            (other_min, other_max)
        } else {
            return Difference {
                inner: DifferenceInner::Iterate(self.iter()),
            };
        };
        Difference {
            inner: match (self_min.cmp(other_max), self_max.cmp(other_min)) {
                (Greater, _) | (_, Less) => DifferenceInner::Iterate(self.iter()),
                (Equal, _) => {
                    let mut self_iter = self.iter();
                    self_iter.next();
                    DifferenceInner::Iterate(self_iter)
                }
                (_, Equal) => {
                    let mut self_iter = self.iter();
                    self_iter.next_back();
                    DifferenceInner::Iterate(self_iter)
                }
                _ if self.len() <= other.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF => {
                    DifferenceInner::Search {
                        self_iter: self.iter(),
                        other_set: other,
                    }
                }
                _ => DifferenceInner::Stitch {
                    self_iter: self.iter(),
                    other_iter: Peeking::new(other.iter()),
                },
            },
        }
    }

    /// Visits the values representing the symmetric difference,
    /// i.e., the values that are in `self` or in `other` but not in both,
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
    /// let sym_diff: Vec<_> = a.symmetric_difference(&b).cloned().collect();
    /// assert_eq!(sym_diff, [1, 3]);
    /// ```
    /*
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn symmetric_difference<'a>(&'a self,
                                    other: &'a BTreeSet<T>)
                                    -> SymmetricDifference<'a, T> {
    */
    fn symmetric_difference<'a>(&'a self, other: &'a BTreeSet<T>) -> SymmetricDifference<'a, T> {
        SymmetricDifference(MergeIter::new(self.iter(), other.iter()))
    }

    /*
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
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn intersection<'a>(&'a self, other: &'a BTreeSet<T>) -> Intersection<'a, T> {
    */
    fn intersection<'a>(&'a self, other: &'a BTreeSet<T>) -> Intersection<'a, T> {
        let (self_min, self_max) = if let (Some(self_min), Some(self_max)) =
            (self.iter().next(), self.iter().next_back())
        {
            (self_min, self_max)
        } else {
            return Intersection {
                inner: IntersectionInner::Answer(None),
            };
        };
        let (other_min, other_max) = if let (Some(other_min), Some(other_max)) =
            (other.iter().next(), other.iter().next_back())
        {
            (other_min, other_max)
        } else {
            return Intersection {
                inner: IntersectionInner::Answer(None),
            };
        };
        Intersection {
            inner: match (self_min.cmp(other_max), self_max.cmp(other_min)) {
                (Greater, _) | (_, Less) => IntersectionInner::Answer(None),
                (Equal, _) => IntersectionInner::Answer(Some(self_min)),
                (_, Equal) => IntersectionInner::Answer(Some(self_max)),
                _ if self.len() <= other.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF => {
                    IntersectionInner::Search {
                        small_iter: self.iter(),
                        large_set: other,
                    }
                }
                _ if other.len() <= self.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF => {
                    IntersectionInner::Search {
                        small_iter: other.iter(),
                        large_set: self,
                    }
                }
                _ => IntersectionInner::Stitch {
                    a: self.iter(),
                    b: other.iter(),
                },
            },
        }
    }

    /*
    /// Visits the values representing the union,
    /// i.e., all the values in `self` or `other`, without duplicates,
    /// in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut a = BTreeSet::new();
    /// a.insert(1);
    ///
    /// let mut b = BTreeSet::new();
    /// b.insert(2);
    ///
    /// let union: Vec<_> = a.union(&b).cloned().collect();
    /// assert_eq!(union, [1, 2]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn union<'a>(&'a self, other: &'a BTreeSet<T>) -> Union<'a, T> {
    */
    fn union<'a>(&'a self, other: &'a BTreeSet<T>) -> Union<'a, T> {
        Union(MergeIter::new(self.iter(), other.iter()))
    }

    /*
    /// Clears the set, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut v = BTreeSet::new();
    /// v.insert(1);
    /// v.clear();
    /// assert!(v.is_empty());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Returns `true` if the set contains a value.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let set: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    /// assert_eq!(set.contains(&1), true);
    /// assert_eq!(set.contains(&4), false);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
        where T: Borrow<Q>,
              Q: Ord
    {
        self.map.contains_key(value)
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let set: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    /// assert_eq!(set.get(&2), Some(&2));
    /// assert_eq!(set.get(&4), None);
    /// ```
    #[stable(feature = "set_recovery", since = "1.9.0")]
    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
        where T: Borrow<Q>,
              Q: Ord
    {
        Recover::get(&self.map, value)
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    /// This is equivalent to checking for an empty intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let a: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    /// let mut b = BTreeSet::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(4);
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(1);
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_disjoint(&self, other: &BTreeSet<T>) -> bool {
        self.intersection(other).next().is_none()
    }

    /// Returns `true` if the set is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let sup: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    /// let mut set = BTreeSet::new();
    ///
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(2);
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(4);
    /// assert_eq!(set.is_subset(&sup), false);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_subset(&self, other: &BTreeSet<T>) -> bool {
    */
    fn is_subset(&self, other: &BTreeSet<T>) -> bool {
        // Same result as self.difference(other).next().is_none()
        // but the code below is faster (hugely in some cases).
        if self.len() > other.len() {
            return false;
        }
        let (self_min, self_max) = if let (Some(self_min), Some(self_max)) =
            (self.iter().next(), self.iter().next_back())
        {
            (self_min, self_max)
        } else {
            return true; // self is empty
        };
        let (other_min, other_max) = if let (Some(other_min), Some(other_max)) =
            (other.iter().next(), other.iter().next_back())
        {
            (other_min, other_max)
        } else {
            return false; // other is empty
        };
        let mut self_iter = self.iter();
        match self_min.cmp(other_min) {
            Less => return false,
            Equal => {
                self_iter.next();
            }
            Greater => (),
        }
        match self_max.cmp(other_max) {
            Greater => return false,
            Equal => {
                self_iter.next_back();
            }
            Less => (),
        }
        if self_iter.len() <= other.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF {
            // Big difference in number of elements.
            for next in self_iter {
                if !other.contains(next) {
                    return false;
                }
            }
        } else {
            // Self is not much smaller than other set.
            let mut other_iter = other.iter();
            other_iter.next();
            other_iter.next_back();
            let mut self_next = self_iter.next();
            while let Some(self1) = self_next {
                match other_iter.next().map_or(Less, |other1| self1.cmp(other1)) {
                    Less => return false,
                    Equal => self_next = self_iter.next(),
                    Greater => (),
                }
            }
        }
        true
    }

    /*
    /// Returns `true` if the set is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let sub: BTreeSet<_> = [1, 2].iter().cloned().collect();
    /// let mut set = BTreeSet::new();
    ///
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(0);
    /// set.insert(1);
    /// assert_eq!(set.is_superset(&sub), false);
    ///
    /// set.insert(2);
    /// assert_eq!(set.is_superset(&sub), true);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_superset(&self, other: &BTreeSet<T>) -> bool {
        other.is_subset(self)
    }

    /// Adds a value to the set.
    ///
    /// If the set did not have this value present, `true` is returned.
    ///
    /// If the set did have this value present, `false` is returned, and the
    /// entry is not updated. See the [module-level documentation] for more.
    ///
    /// [module-level documentation]: index.html#insert-and-complex-keys
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut set = BTreeSet::new();
    ///
    /// assert_eq!(set.insert(2), true);
    /// assert_eq!(set.insert(2), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }

    /// Adds a value to the set, replacing the existing value, if any, that is equal to the given
    /// one. Returns the replaced value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut set = BTreeSet::new();
    /// set.insert(Vec::<i32>::new());
    ///
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
    /// set.replace(Vec::with_capacity(10));
    /// assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    /// ```
    #[stable(feature = "set_recovery", since = "1.9.0")]
    pub fn replace(&mut self, value: T) -> Option<T> {
        Recover::replace(&mut self.map, value)
    }

    /// Removes a value from the set. Returns whether the value was
    /// present in the set.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut set = BTreeSet::new();
    ///
    /// set.insert(2);
    /// assert_eq!(set.remove(&2), true);
    /// assert_eq!(set.remove(&2), false);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
        where T: Borrow<Q>,
              Q: Ord
    {
        self.map.remove(value).is_some()
    }

    /// Removes and returns the value in the set, if any, that is equal to the given one.
    ///
    /// The value may be any borrowed form of the set's value type,
    /// but the ordering on the borrowed form *must* match the
    /// ordering on the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut set: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    /// assert_eq!(set.take(&2), Some(2));
    /// assert_eq!(set.take(&2), None);
    /// ```
    #[stable(feature = "set_recovery", since = "1.9.0")]
    pub fn take<Q: ?Sized>(&mut self, value: &Q) -> Option<T>
        where T: Borrow<Q>,
              Q: Ord
    {
        Recover::take(&mut self.map, value)
    }

    /// Moves all elements from `other` into `Self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut a = BTreeSet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    ///
    /// let mut b = BTreeSet::new();
    /// b.insert(3);
    /// b.insert(4);
    /// b.insert(5);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 5);
    /// assert_eq!(b.len(), 0);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    /// assert!(a.contains(&3));
    /// assert!(a.contains(&4));
    /// assert!(a.contains(&5));
    /// ```
    #[stable(feature = "btree_append", since = "1.11.0")]
    pub fn append(&mut self, other: &mut Self) {
        self.map.append(&mut other.map);
    }

    /// Splits the collection into two at the given key. Returns everything after the given key,
    /// including the key.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut a = BTreeSet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    /// a.insert(17);
    /// a.insert(41);
    ///
    /// let b = a.split_off(&3);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    ///
    /// assert!(a.contains(&1));
    /// assert!(a.contains(&2));
    ///
    /// assert!(b.contains(&3));
    /// assert!(b.contains(&17));
    /// assert!(b.contains(&41));
    /// ```
    #[stable(feature = "btree_split_off", since = "1.11.0")]
    pub fn split_off<Q: ?Sized + Ord>(&mut self, key: &Q) -> Self where T: Borrow<Q> {
        BTreeSet { map: self.map.split_off(key) }
    }
    */
}

/*
impl<T> BTreeSet<T> {
    /// Gets an iterator that visits the values in the `BTreeSet` in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let set: BTreeSet<usize> = [1, 2, 3].iter().cloned().collect();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), Some(&1));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&3));
    /// assert_eq!(set_iter.next(), None);
    /// ```
    ///
    /// Values returned by the iterator are returned in ascending order:
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let set: BTreeSet<usize> = [3, 1, 2].iter().cloned().collect();
    /// let mut set_iter = set.iter();
    /// assert_eq!(set_iter.next(), Some(&1));
    /// assert_eq!(set_iter.next(), Some(&2));
    /// assert_eq!(set_iter.next(), Some(&3));
    /// assert_eq!(set_iter.next(), None);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { iter: self.map.keys() }
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut v = BTreeSet::new();
    /// assert_eq!(v.len(), 0);
    /// v.insert(1);
    /// assert_eq!(v.len(), 1);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let mut v = BTreeSet::new();
    /// assert!(v.is_empty());
    /// v.insert(1);
    /// assert!(!v.is_empty());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord> FromIterator<T> for BTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BTreeSet<T> {
        let mut set = BTreeSet::new();
        set.extend(iter);
        set
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> IntoIterator for BTreeSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Gets an iterator for moving out the `BTreeSet`'s contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let set: BTreeSet<usize> = [1, 2, 3, 4].iter().cloned().collect();
    ///
    /// let v: Vec<_> = set.into_iter().collect();
    /// assert_eq!(v, [1, 2, 3, 4]);
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { iter: self.map.into_iter() }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> IntoIterator for &'a BTreeSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord> Extend<T> for BTreeSet<T> {
    #[inline]
    fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
        iter.into_iter().for_each(move |elem| {
            self.insert(elem);
        });
    }
}

#[stable(feature = "extend_ref", since = "1.2.0")]
impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for BTreeSet<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord> Default for BTreeSet<T> {
    /// Makes an empty `BTreeSet<T>` with a reasonable choice of B.
    fn default() -> BTreeSet<T> {
        BTreeSet::new()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord + Clone> Sub<&BTreeSet<T>> for &BTreeSet<T> {
    type Output = BTreeSet<T>;

    /// Returns the difference of `self` and `rhs` as a new `BTreeSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let a: BTreeSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: BTreeSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let result = &a - &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [1, 2]);
    /// ```
    fn sub(self, rhs: &BTreeSet<T>) -> BTreeSet<T> {
        self.difference(rhs).cloned().collect()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord + Clone> BitXor<&BTreeSet<T>> for &BTreeSet<T> {
    type Output = BTreeSet<T>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `BTreeSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let a: BTreeSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: BTreeSet<_> = vec![2, 3, 4].into_iter().collect();
    ///
    /// let result = &a ^ &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [1, 4]);
    /// ```
    fn bitxor(self, rhs: &BTreeSet<T>) -> BTreeSet<T> {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord + Clone> BitAnd<&BTreeSet<T>> for &BTreeSet<T> {
    type Output = BTreeSet<T>;

    /// Returns the intersection of `self` and `rhs` as a new `BTreeSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let a: BTreeSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: BTreeSet<_> = vec![2, 3, 4].into_iter().collect();
    ///
    /// let result = &a & &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [2, 3]);
    /// ```
    fn bitand(self, rhs: &BTreeSet<T>) -> BTreeSet<T> {
        self.intersection(rhs).cloned().collect()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord + Clone> BitOr<&BTreeSet<T>> for &BTreeSet<T> {
    type Output = BTreeSet<T>;

    /// Returns the union of `self` and `rhs` as a new `BTreeSet<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    ///
    /// let a: BTreeSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: BTreeSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let result = &a | &b;
    /// let result_vec: Vec<_> = result.into_iter().collect();
    /// assert_eq!(result_vec, [1, 2, 3, 4, 5]);
    /// ```
    fn bitor(self, rhs: &BTreeSet<T>) -> BTreeSet<T> {
        self.union(rhs).cloned().collect()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Debug> Debug for BTreeSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter { iter: self.iter.clone() }
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize { self.iter.len() }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Iter<'_, T> {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next().map(|(k, _)| k)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back().map(|(k, _)| k)
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize { self.iter.len() }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for IntoIter<T> {}

#[stable(feature = "btree_range", since = "1.17.0")]
impl<T> Clone for Range<'_, T> {
    fn clone(&self) -> Self {
        Range { iter: self.iter.clone() }
    }
}

#[stable(feature = "btree_range", since = "1.17.0")]
impl<'a, T> Iterator for Range<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next().map(|(k, _)| k)
    }

    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

#[stable(feature = "btree_range", since = "1.17.0")]
impl<'a, T> DoubleEndedIterator for Range<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back().map(|(k, _)| k)
    }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<T> FusedIterator for Range<'_, T> {}

#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<T> Clone for Difference<'_, T> {
    fn clone(&self) -> Self {
        Difference {
            inner: match &self.inner {
                DifferenceInner::Stitch {
                    self_iter,
                    other_iter,
                } => DifferenceInner::Stitch {
                    self_iter: self_iter.clone(),
                    other_iter: other_iter.clone(),
                },
                DifferenceInner::Search {
                    self_iter,
                    other_set,
                } => DifferenceInner::Search {
                    self_iter: self_iter.clone(),
                    other_set,
                },
                DifferenceInner::Iterate(iter) => DifferenceInner::Iterate(iter.clone()),
            },
        }
    }
}
/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<'a, T: Ord> Iterator for Difference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match &mut self.inner {
            DifferenceInner::Stitch {
                self_iter,
                other_iter,
            } => {
                let mut self_next = self_iter.next()?;
                loop {
                    match other_iter
                        .peek()
                        .map_or(Less, |other_next| self_next.cmp(other_next))
                    {
                        Less => return Some(self_next),
                        Equal => {
                            self_next = self_iter.next()?;
                            other_iter.next();
                        }
                        Greater => {
                            other_iter.next();
                        }
                    }
                }
            }
            DifferenceInner::Search {
                self_iter,
                other_set,
            } => loop {
                let self_next = self_iter.next()?;
                if !other_set.contains(&self_next) {
                    return Some(self_next);
                }
            },
            DifferenceInner::Iterate(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (self_len, other_len) = match &self.inner {
            DifferenceInner::Stitch {
                self_iter,
                other_iter,
            } => (self_iter.len(), other_iter.len()),
            DifferenceInner::Search {
                self_iter,
                other_set,
            } => (self_iter.len(), other_set.len()),
            DifferenceInner::Iterate(iter) => (iter.len(), 0),
        };
        (self_len.saturating_sub(other_len), Some(self_len))
    }
}

/*
#[stable(feature = "fused", since = "1.26.0")]
impl<T: Ord> FusedIterator for Difference<'_, T> {}

#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<T> Clone for SymmetricDifference<'_, T> {
    fn clone(&self) -> Self {
        SymmetricDifference(self.0.clone())
    }
}
/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<'a, T: Ord> Iterator for SymmetricDifference<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            let (next, count) = self.0.next_counted();
            if count <= 1 {
                return next;
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.0.a.len() + self.0.b.len()))
    }
}

/*
#[stable(feature = "fused", since = "1.26.0")]
impl<T: Ord> FusedIterator for SymmetricDifference<'_, T> {}

#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<T> Clone for Intersection<'_, T> {
    fn clone(&self) -> Self {
        Intersection {
            inner: match &self.inner {
                IntersectionInner::Stitch {
                    a,
                    b,
                } => IntersectionInner::Stitch {
                    a: a.clone(),
                    b: b.clone(),
                },
                IntersectionInner::Search {
                    small_iter,
                    large_set,
                } => IntersectionInner::Search {
                    small_iter: small_iter.clone(),
                    large_set,
                },
                IntersectionInner::Answer(answer) => IntersectionInner::Answer(answer.clone()),
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
            IntersectionInner::Stitch {
                a,
                b,
            } => {
                let mut a_next = a.next()?;
                let mut b_next = b.next()?;
                loop {
                    match a_next.cmp(b_next) {
                        Less => a_next = a.next()?,
                        Greater => b_next = b.next()?,
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
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            IntersectionInner::Stitch { a, b } => (0, Some(min(a.len(), b.len()))),
            IntersectionInner::Search { small_iter, .. } => (0, Some(small_iter.len())),
            IntersectionInner::Answer(None) => (0, Some(0)),
            IntersectionInner::Answer(Some(_)) => (1, Some(1)),
        }
    }
}

/*
#[stable(feature = "fused", since = "1.26.0")]
impl<T: Ord> FusedIterator for Intersection<'_, T> {}

#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<T> Clone for Union<'_, T> {
    fn clone(&self) -> Self {
        Union(self.0.clone())
    }
}
/*
#[stable(feature = "rust1", since = "1.0.0")]
*/
impl<'a, T: Ord> Iterator for Union<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let a_len = self.0.a.len();
        let b_len = self.0.b.len();
        (max(a_len, b_len), Some(a_len + b_len))
    }
}

/*
#[stable(feature = "fused", since = "1.26.0")]
impl<T: Ord> FusedIterator for Union<'_, T> {}
*/

pub struct IntersectionSwitch<'a, T: 'a> {
    inner: IntersectionSwitchInner<'a, T>,
}
enum IntersectionSwitchInner<'a, T: 'a> {
    Stitch {
        // iterate both sets jointly, or iterate one and look up in the other
        a_iter: Iter<'a, T>,
        a_set: &'a BTreeSet<T>,
        b_iter: Iter<'a, T>,
        b_set: &'a BTreeSet<T>,
    },
    Answer(Option<&'a T>), // return a specific value or emptiness
}

impl<T: fmt::Debug> fmt::Debug for IntersectionSwitch<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            IntersectionSwitchInner::Stitch {
                a_iter,
                a_set,
                b_iter,
                b_set,
            } => f
                .debug_tuple("Intersection")
                .field(&a_iter)
                .field(&a_set)
                .field(&b_iter)
                .field(&b_set)
                .finish(),
            IntersectionSwitchInner::Answer(answer) => {
                f.debug_tuple("Intersection").field(&answer).finish()
            }
        }
    }
}

impl<'a, T: Ord> Iterator for IntersectionSwitch<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        fn search_remainder<'b, S: Ord>(
            small_iter: &mut Iter<'b, S>,
            large_iter: &Iter<'b, S>,
            large_set: &BTreeSet<S>,
        ) -> Option<Option<&'b S>> {
            if small_iter.len() > large_iter.len() / ITER_PERFORMANCE_TIPPING_SIZE_DIFF {
                None
            } else {
                // At this point, large_iter's position may be one iteration
                // beyond what you'd assume, and remains stuck, but it won't
                // be used anymore. large_iter's length remains large, so we
                // will keep coming back here, and it won't spoil size_hint.
                while let Some(next) = small_iter.next() {
                    if large_set.contains(&next) {
                        return Some(Some(next));
                    }
                }
                Some(None)
            }
        }

        match &mut self.inner {
            IntersectionSwitchInner::Stitch {
                a_set,
                b_set,
                a_iter,
                b_iter,
            } => {
                if let Some(result) = search_remainder(a_iter, b_iter, b_set) {
                    return result;
                }
                if let Some(result) = search_remainder(b_iter, a_iter, a_set) {
                    return result;
                }
                let mut a_next = a_iter.next()?;
                let mut b_next = b_iter.next()?;
                loop {
                    match a_next.cmp(b_next) {
                        Less => {
                            if let Some(result) = search_remainder(a_iter, b_iter, b_set) {
                                return result;
                            }
                            a_next = a_iter.next()?
                        }
                        Greater => {
                            if let Some(result) = search_remainder(b_iter, a_iter, a_set) {
                                return result;
                            }
                            b_next = b_iter.next()?
                        }
                        Equal => return Some(a_next),
                    }
                }
            }
            IntersectionSwitchInner::Answer(answer) => answer.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            IntersectionSwitchInner::Stitch { a_iter, b_iter, .. } => {
                (0, Some(min(a_iter.len(), b_iter.len())))
            }
            IntersectionSwitchInner::Answer(None) => (0, Some(0)),
            IntersectionSwitchInner::Answer(Some(_)) => (1, Some(1)),
        }
    }
}

impl<T> Clone for IntersectionSwitch<'_, T> {
    fn clone(&self) -> Self {
        IntersectionSwitch {
            inner: match &self.inner {
                IntersectionSwitchInner::Stitch {
                    a_set,
                    b_set,
                    a_iter,
                    b_iter,
                } => IntersectionSwitchInner::Stitch {
                    a_set: a_set,
                    b_set: b_set,
                    a_iter: a_iter.clone(),
                    b_iter: b_iter.clone(),
                },
                IntersectionSwitchInner::Answer(answer) => {
                    IntersectionSwitchInner::Answer(answer.clone())
                }
            },
        }
    }
}

pub struct IntersectionSwivel<'a, T: 'a> {
    a_range: Range<'a, T>,
    b_range: Range<'a, T>,
    a_set: &'a BTreeSet<T>,
    b_set: &'a BTreeSet<T>,
}

impl<'a, T: Ord> Iterator for IntersectionSwivel<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        const NEXT_COUNT_MAX: usize = ITER_PERFORMANCE_TIPPING_SIZE_DIFF;
        let mut next_count: usize = 0;
        let mut a_next = self.a_range.next()?;
        let mut b_next = self.b_range.next()?;
        loop {
            match a_next.cmp(b_next) {
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

pub fn is_subset_future<'a, T: Ord>(selve: &'a BTreeSet<T>, other: &'a BTreeSet<T>) -> bool {
    (selve as &dyn JustToIndentAsMuch<T>).is_subset(other)
}

pub fn difference_future<'a, T: Ord>(
    selve: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Difference<'a, T> {
    (selve as &dyn JustToIndentAsMuch<T>).difference(other)
}

pub fn intersection_future<'a, T: Ord>(
    selve: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> Intersection<'a, T> {
    (selve as &dyn JustToIndentAsMuch<T>).intersection(other)
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
            a: a.iter(),
            b: b.iter(),
        },
    }
}

pub fn intersection_switch<'a, T: Ord>(
    selve: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> IntersectionSwitch<'a, T> {
    let (self_min, self_max) =
        if let (Some(self_min), Some(self_max)) = (selve.iter().next(), selve.iter().next_back()) {
            (self_min, self_max)
        } else {
            return IntersectionSwitch {
                inner: IntersectionSwitchInner::Answer(None),
            };
        };
    let (other_min, other_max) = if let (Some(other_min), Some(other_max)) =
        (other.iter().next(), other.iter().next_back())
    {
        (other_min, other_max)
    } else {
        return IntersectionSwitch {
            inner: IntersectionSwitchInner::Answer(None),
        };
    };
    IntersectionSwitch {
        inner: match (self_min.cmp(other_max), self_max.cmp(other_min)) {
            (Greater, _) | (_, Less) => IntersectionSwitchInner::Answer(None),
            (Equal, _) => IntersectionSwitchInner::Answer(Some(self_min)),
            (_, Equal) => IntersectionSwitchInner::Answer(Some(self_max)),
            _ => IntersectionSwitchInner::Stitch {
                a_iter: selve.iter(),
                a_set: selve,
                b_iter: other.iter(),
                b_set: other,
            },
        },
    }
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

pub fn symmdiff_future<'a, T: Ord>(
    selve: &'a BTreeSet<T>,
    other: &'a BTreeSet<T>,
) -> SymmetricDifference<'a, T> {
    (selve as &dyn JustToIndentAsMuch<T>).symmetric_difference(other)
}

pub fn union_future<'a, T: Ord>(selve: &'a BTreeSet<T>, other: &'a BTreeSet<T>) -> Union<'a, T> {
    (selve as &dyn JustToIndentAsMuch<T>).union(other)
}
