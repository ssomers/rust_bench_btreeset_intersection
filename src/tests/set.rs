extern crate proptest;
use self::proptest::prelude::*;
use ::rust_bench_btreeset_intersection::set::{
    difference_future, intersection_future, intersection_swivel, is_subset_future,
};
use std::collections::BTreeSet;

fn assert_difference<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a BTreeSet<u8>,
    s2: &'a BTreeSet<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt));
        prop_assert!(!s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(s1.iter().filter(|elt| !s2.contains(elt)).count(), count);
    Ok(())
}

fn assert_intersection<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a BTreeSet<u8>,
    s2: &'a BTreeSet<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt));
        prop_assert!(s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(s1.iter().filter(|elt| s2.contains(elt)).count(), count);
    Ok(())
}

prop_compose! {
    fn aligned_ranges()
                     (mut s1: BTreeSet<u8>,
                      mut s2: BTreeSet<u8>)
                     -> (BTreeSet<u8>, BTreeSet<u8>)
    {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        (s1, s2)
    }
}

prop_compose! {
    fn left_aligned_ranges()
                          (mut s1: BTreeSet<u8>,
                           mut s2: BTreeSet<u8>)
                          -> (BTreeSet<u8>, BTreeSet<u8>)
    {
        s1.insert(u8::min_value());
        s2.insert(u8::min_value());
        (s1, s2)
    }
}

prop_compose! {
    fn right_aligned_ranges()
                           (mut s1: BTreeSet<u8>,
                            mut s2: BTreeSet<u8>)
                           -> (BTreeSet<u8>, BTreeSet<u8>)
    {
        s1.insert(u8::max_value());
        s2.insert(u8::max_value());
        (s1, s2)
    }
}

prop_compose! {
    fn disjoint_ranges()
                      (mut s1: BTreeSet<u8>,
                       split: u8,
                       right_then_left: bool)
                      -> (BTreeSet<u8>, BTreeSet<u8>)
    {
        let s2 = s1.split_off(&split);
        if right_then_left { (s2, s1) } else { (s1, s2) }
    }
}

prop_compose! {
    fn touching_ranges()
                      (mut s1: BTreeSet<u8>,
                       split: u8,
                       right_then_left: bool)
                      -> (BTreeSet<u8>, BTreeSet<u8>)
    {
        let mut s2 = s1.split_off(&split);
        s1.insert(split);
        s2.insert(split);
        if right_then_left { (s2, s1) } else { (s1, s2) }
    }
}

proptest! {
    #[test]
    fn difference_future_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_aligned_left((s1, s2) in left_aligned_ranges()) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_aligned_right((s1, s2) in right_aligned_ranges()) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_aligned_both((s1, s2) in aligned_ranges()) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_disjoint((s1, s2) in disjoint_ranges()) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn difference_future_touching((s1, s2) in touching_ranges()) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn is_subset_future_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        prop_assert_eq!(s1.is_subset(&s2), is_subset_future(&s1, &s2));
    }

    #[test]
    fn is_subset_future_aligned_left((s1, s2) in left_aligned_ranges()) {
        prop_assert_eq!(s1.is_subset(&s2), is_subset_future(&s1, &s2));
    }

    #[test]
    fn is_subset_future_aligned_right((s1, s2) in right_aligned_ranges()) {
        prop_assert_eq!(s1.is_subset(&s2), is_subset_future(&s1, &s2));
    }

    #[test]
    fn is_subset_future_aligned_both((s1, s2) in aligned_ranges()) {
        prop_assert_eq!(s1.is_subset(&s2), is_subset_future(&s1, &s2));
    }

    #[test]
    fn is_subset_future_disjoint((s1, s2) in disjoint_ranges()) {
        prop_assert_eq!(s1.is_subset(&s2), is_subset_future(&s1, &s2));
    }

    #[test]
    fn is_subset_future_touching((s2, s1) in touching_ranges()) {
        prop_assert_eq!(s1.is_subset(&s2), is_subset_future(&s1, &s2));
    }

    #[test]
    fn intersection_future_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_aligned_left((s1, s2) in left_aligned_ranges()) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_aligned_right((s1, s2) in right_aligned_ranges()) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_aligned_both((s1, s2) in aligned_ranges()) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_disjoint1((s1, s2) in disjoint_ranges()) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_future_touching((s2, s1) in touching_ranges()) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_swivel_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?;
    }

    #[test]
    fn intersection_swivel_aligned_left((s1, s2) in left_aligned_ranges()) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_swivel_aligned_right((s1, s2) in right_aligned_ranges()) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_swivel_aligned_both((s1, s2) in aligned_ranges()) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_swivel_disjoint1((s1, s2) in disjoint_ranges()) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn intersection_swivel_touching((s2, s1) in touching_ranges()) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?
    }
}
