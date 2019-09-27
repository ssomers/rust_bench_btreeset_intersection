extern crate proptest;
extern crate rand;
extern crate rand_xorshift;
use self::proptest::prelude::*;
use ::rust_bench_btreeset_intersection::set::*;
use std::collections::BTreeSet;

fn assert_difference<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a BTreeSet<u8>,
    s2: &'a BTreeSet<u8>,
) -> Result<(), TestCaseError> {
    let mut last: i32 = -1;
    let mut collected = BTreeSet::<u8>::new();
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt));
        prop_assert!(!s2.contains(&elt));
        prop_assert!(i32::from(elt) > last);
        last = i32::from(elt);
        collected.insert(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    for elt in s1 {
        if !s2.contains(elt) {
            prop_assert!(collected.contains(elt));
        }
    }
    Ok(())
}

fn assert_intersection<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a BTreeSet<u8>,
    s2: &'a BTreeSet<u8>,
) -> Result<(), TestCaseError> {
    let mut last: i32 = -1;
    let mut collected = BTreeSet::<u8>::new();
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt));
        prop_assert!(s2.contains(&elt));
        prop_assert!(i32::from(elt) > last);
        last = i32::from(elt);
        collected.insert(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    for elt in s1 {
        if s2.contains(elt) {
            prop_assert!(collected.contains(elt));
        }
    }
    for elt in s2 {
        if s1.contains(elt) {
            prop_assert!(collected.contains(elt));
        }
    }
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 8192, .. ProptestConfig::default()
    })]

    #[test]
    fn difference_future_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_aligned_left(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_aligned_right(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_aligned_both(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn is_subset_future_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        prop_assert_eq!(difference_future(&s1, &s2).next().is_none(),
                        is_subset_future(&s1, &s2))
    }

    #[test]
    fn is_subset_future_aligned_left(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        prop_assert_eq!(difference_future(&s1, &s2).next().is_none(),
                        is_subset_future(&s1, &s2))
    }

    #[test]
    fn is_subset_future_aligned_right(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        prop_assert_eq!(difference_future(&s1, &s2).next().is_none(),
                        is_subset_future(&s1, &s2))
    }

    #[test]
    fn is_subset_future_aligned_both(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        prop_assert_eq!(difference_future(&s1, &s2).next().is_none(),
                        is_subset_future(&s1, &s2))
    }

    #[test]
    fn intersection_future_aligned_left(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_aligned_right(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_aligned_both(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>) {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_disjoint(mut s1: BTreeSet<u8>, mut s2: BTreeSet<u8>, split: u8) {
        while let Some(&max) = s1.iter().next_back() {
            if max > split {
                s1.remove(&max);
            } else {
                break
            }
        }
        while let Some(&min) = s2.iter().next() {
            if min < split {
                s2.remove(&min);
            } else {
                break
            }
        }
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_swivel_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?;
    }
}
