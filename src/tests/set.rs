extern crate proptest;
extern crate rand;
extern crate rand_xorshift;
use self::proptest::prelude::*;
use self::rand::Rng;
use self::rand::SeedableRng;
use self::rand_xorshift::XorShiftRng;
use ::rust_bench_btreeset_intersection::set::*;
use std::collections::BTreeSet;

fn random_set(size: usize, ovule: u8) -> BTreeSet<usize> {
    let mut rng = XorShiftRng::from_seed([ovule; 16]);
    let mut s = BTreeSet::<usize>::new();
    while s.len() < size {
        s.insert(rng.gen());
    }
    s
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
    #[test]
    fn test_intersection_future(len1 in 0..1000usize, len2 in 0..1000usize) {
        prop_assume!(len1 <= len2);
        let s1 = random_set(len1, 11u8);
        let s2 = random_set(len2, 22u8);
        assert_intersection(intersection_future(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn test_intersection_search(len1 in 0..1000usize, len2 in 0..1000usize) {
        prop_assume!(len1 <= len2);
        let s1 = random_set(len1, 11u8);
        let s2 = random_set(len2, 22u8);
        assert_intersection(intersection_search(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn test_intersection_spring(len1 in 0..1000usize, len2 in 0..1000usize) {
        prop_assume!(len1 <= len2);
        let s1 = random_set(len1, 11u8);
        let s2 = random_set(len2, 22u8);
        assert_intersection(intersection_spring(&s1, &s2), &s1, &s2)?
    }

    #[test]
    fn test_intersection_stitch(len1 in 0..1000usize, len2 in 0..1000usize) {
        prop_assume!(len1 <= len2);
        let s1 = random_set(len1, 11u8);
        let s2 = random_set(len2, 22u8);
        assert_intersection(intersection_stitch(&s1, &s2), &s1, &s2)?
    }
}
