extern crate proptest;
extern crate rand;
extern crate rand_xorshift;
use self::proptest::prelude::*;
use ::rust_bench_btreeset_intersection::set::*;
use std::collections::BTreeSet;

fn assert_difference<'a, I: Iterator<Item = &'a usize>>(
    it: I,
    s1: &'a BTreeSet<usize>,
    s2: &'a BTreeSet<usize>,
) -> Result<(), TestCaseError> {
    let mut collected = BTreeSet::<usize>::new();
    for elt in it {
        prop_assert!(s1.contains(elt));
        prop_assert!(!s2.contains(elt));
        prop_assert!(collected.insert(*elt));
    }
    for elt in s1 {
        prop_assert_eq!(collected.contains(elt), !s2.contains(elt));
    }
    Ok(())
}


fn assert_intersection<'a, I: Iterator<Item = &'a usize>>(
    it: I,
    s1: &'a BTreeSet<usize>,
    s2: &'a BTreeSet<usize>,
) -> Result<(), TestCaseError> {
    let mut collected = BTreeSet::<usize>::new();
    for elt in it {
        prop_assert!(s1.contains(elt));
        prop_assert!(s2.contains(elt));
        prop_assert!(collected.insert(*elt));
    }
    for elt in s1 {
        prop_assert_eq!(collected.contains(elt), s2.contains(elt));
    }
    for elt in s2 {
        prop_assert_eq!(collected.contains(elt), s1.contains(elt));
    }
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10_000, .. ProptestConfig::default()
    })]

    #[test]
    fn test_difference_future(s1: BTreeSet<usize>, s2: BTreeSet<usize>) {
        assert_difference(difference_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn test_is_subset_future(s1: BTreeSet<usize>, s2: BTreeSet<usize>) {
        prop_assert_eq!(difference_future(&s1, &s2).next().is_none(),
                        is_subset_future(&s1, &s2));
    }

    #[test]
    fn test_intersection_future(s1: BTreeSet<usize>, s2: BTreeSet<usize>) {
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn test_intersection_swivel(s1: BTreeSet<usize>, s2: BTreeSet<usize>) {
        assert_intersection(intersection_swivel(&s1, &s2), &s1, &s2)?
    }
}
