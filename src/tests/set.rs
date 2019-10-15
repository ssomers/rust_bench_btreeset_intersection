extern crate proptest;
use self::proptest::prelude::*;
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
    prop_assert_eq!(count, s1.iter().filter(|elt| !s2.contains(elt)).count());
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
    prop_assert_eq!(count, s1.iter().filter(|elt| s2.contains(elt)).count());
    Ok(())
}

fn assert_symmdiff<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a BTreeSet<u8>,
    s2: &'a BTreeSet<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert_eq!(s1.contains(&elt), !s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(
        count,
        s1.len() + s2.len() - 2 * s1.iter().filter(|elt| s2.contains(elt)).count()
    );
    Ok(())
}

fn assert_union<'a, I: Iterator<Item = &'a u8>>(
    mut it: I,
    s1: &'a BTreeSet<u8>,
    s2: &'a BTreeSet<u8>,
) -> Result<(), TestCaseError> {
    let mut count: usize = 0;
    let mut previous: i32 = -1;
    while let Some(&elt) = it.next() {
        prop_assert!(s1.contains(&elt) || s2.contains(&elt));
        prop_assert!(i32::from(elt) > previous);
        count += 1;
        previous = i32::from(elt);
    }
    for _ in 0..42 {
        prop_assert!(it.next().is_none()); // it's announced to be a fused iterator
    }
    prop_assert_eq!(
        count,
        s1.len() + s2.len() - s1.iter().filter(|elt| s2.contains(elt)).count()
    );
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
                       right_then_left: bool)
                      -> (BTreeSet<u8>, BTreeSet<u8>)
    {
        let split = (u8::max_value() - u8::min_value()) / 2;
        let mut s2 = s1.split_off(&split);
        s1.insert(u8::min_value());
        s2.insert(u8::max_value());
        if right_then_left { (s2, s1) } else { (s1, s2) }
    }
}

prop_compose! {
    fn touching_ranges()
                      (mut s1: BTreeSet<u8>,
                       right_then_left: bool)
                      -> (BTreeSet<u8>, BTreeSet<u8>)
    {
        let split = (u8::max_value() - u8::min_value()) / 2;
        let mut s2 = s1.split_off(&split);
        s1.insert(split);
        s2.insert(split);
        if right_then_left { (s2, s1) } else { (s1, s2) }
    }
}

macro_rules! set_tests {
    ($test_mod_name: ident, $mod_name: ident) => {
        mod $test_mod_name {
            use rust_bench_btreeset::$mod_name;
            use std::collections::BTreeSet;
            super::proptest! {
                #[test]
                fn difference_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
                    super::assert_difference($mod_name::difference(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn difference_aligned_left((s1, s2) in super::left_aligned_ranges()) {
                    super::assert_difference($mod_name::difference(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn difference_aligned_right((s1, s2) in super::right_aligned_ranges()) {
                    super::assert_difference($mod_name::difference(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn difference_aligned_both((s1, s2) in super::aligned_ranges()) {
                    super::assert_difference($mod_name::difference(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn difference_disjoint((s1, s2) in super::disjoint_ranges()) {
                    super::assert_difference($mod_name::difference(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn difference_touching((s1, s2) in super::touching_ranges()) {
                    super::assert_difference($mod_name::difference(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn is_subset_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
                    super::prop_assert_eq!(s1.is_subset(&s2), $mod_name::is_subset(&s1, &s2));
                }

                #[test]
                fn is_subset_aligned_left((s1, s2) in super::left_aligned_ranges()) {
                    super::prop_assert_eq!(s1.is_subset(&s2), $mod_name::is_subset(&s1, &s2));
                }

                #[test]
                fn is_subset_aligned_right((s1, s2) in super::right_aligned_ranges()) {
                    super::prop_assert_eq!(s1.is_subset(&s2), $mod_name::is_subset(&s1, &s2));
                }

                #[test]
                fn is_subset_aligned_both((s1, s2) in super::aligned_ranges()) {
                    super::prop_assert_eq!(s1.is_subset(&s2), $mod_name::is_subset(&s1, &s2));
                }

                #[test]
                fn is_subset_disjoint((s1, s2) in super::disjoint_ranges()) {
                    super::prop_assert_eq!(s1.is_subset(&s2), $mod_name::is_subset(&s1, &s2));
                }

                #[test]
                fn is_subset_touching((s2, s1) in super::touching_ranges()) {
                    super::prop_assert_eq!(s1.is_subset(&s2), $mod_name::is_subset(&s1, &s2));
                }

                #[test]
                fn intersection_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
                    super::assert_intersection($mod_name::intersection(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn intersection_aligned_left((s1, s2) in super::left_aligned_ranges()) {
                    super::assert_intersection($mod_name::intersection(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn intersection_aligned_right((s1, s2) in super::right_aligned_ranges()) {
                    super::assert_intersection($mod_name::intersection(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn intersection_aligned_both((s1, s2) in super::aligned_ranges()) {
                    super::assert_intersection($mod_name::intersection(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn intersection_disjoint1((s1, s2) in super::disjoint_ranges()) {
                    super::assert_intersection($mod_name::intersection(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn intersection_touching((s2, s1) in super::touching_ranges()) {
                    super::assert_intersection($mod_name::intersection(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn symmdiff_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
                    super::assert_symmdiff($mod_name::symmdiff(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn symmdiff_aligned_left((s1, s2) in super::left_aligned_ranges()) {
                    super::assert_symmdiff($mod_name::symmdiff(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn symmdiff_aligned_right((s1, s2) in super::right_aligned_ranges()) {
                    super::assert_symmdiff($mod_name::symmdiff(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn symmdiff_aligned_both((s1, s2) in super::aligned_ranges()) {
                    super::assert_symmdiff($mod_name::symmdiff(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn symmdiff_disjoint1((s1, s2) in super::disjoint_ranges()) {
                    super::assert_symmdiff($mod_name::symmdiff(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn symmdiff_touching((s2, s1) in super::touching_ranges()) {
                    super::assert_symmdiff($mod_name::symmdiff(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn union_arbitrary(s1: BTreeSet<u8>, s2: BTreeSet<u8>) {
                    super::assert_union($mod_name::union(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn union_aligned_left((s1, s2) in super::left_aligned_ranges()) {
                    super::assert_union($mod_name::union(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn union_aligned_right((s1, s2) in super::right_aligned_ranges()) {
                    super::assert_union($mod_name::union(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn union_aligned_both((s1, s2) in super::aligned_ranges()) {
                    super::assert_union($mod_name::union(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn union_disjoint1((s1, s2) in super::disjoint_ranges()) {
                    super::assert_union($mod_name::union(&s1, &s2), &s1, &s2)?
                }

                #[test]
                fn union_touching((s2, s1) in super::touching_ranges()) {
                    super::assert_union($mod_name::union(&s1, &s2), &s1, &s2)?
                }
            }
        }
    };
}

set_tests! {test_now, set_now}
set_tests! {test_mergeiter, set_mergeiter}
set_tests! {test_peeking, set_peeking}
set_tests! {test_switch, set_switch}
set_tests! {test_swivel, set_swivel}
