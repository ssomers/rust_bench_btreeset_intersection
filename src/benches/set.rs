// file comparable to rust/src/liballoc/benches/btree/set.rs
// Or it used to be.
#![feature(test)]

extern crate rand;
extern crate rand_xorshift;
extern crate test;
use self::rand::{Rng, SeedableRng};
use self::rand_xorshift::XorShiftRng;
use std::collections::BTreeSet;

fn random(n1: usize, n2: usize) -> [BTreeSet<usize>; 2] {
    let mut rng = XorShiftRng::from_seed([68; 16]);
    let mut sets = [BTreeSet::new(), BTreeSet::new()];
    for i in 0..2 {
        while sets[i].len() < [n1, n2][i] {
            sets[i].insert(rng.gen());
        }
    }
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

fn neg(n: usize) -> BTreeSet<i32> {
    let mut set = BTreeSet::new();
    for i in -(n as i32)..=-1 {
        set.insert(i);
    }
    assert_eq!(set.len(), n);
    set
}

fn pos(n: usize) -> BTreeSet<i32> {
    let mut set = BTreeSet::new();
    for i in 1..=(n as i32) {
        set.insert(i);
    }
    assert_eq!(set.len(), n);
    set
}

fn subsets(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
    let n2 = n1 * factor;
    let mut sets = [BTreeSet::new(), BTreeSet::new()];
    for elt in 0..n2 {
        if ((n2 - n1) / 2..(n2 + n1) / 2).contains(&elt) {
            sets[0].insert(elt as u32);
        }
        sets[1].insert(elt as u32);
    }
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

#[cfg(feature = "stagger")]
fn stagger(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
    use std::cmp::min;
    let n2 = n1 * factor;
    let mut sets = [BTreeSet::new(), BTreeSet::new()];
    for elt in 0..(n1 + n2) {
        let i = min(1, elt % (factor + 1));
        sets[i].insert(elt as u32);
    }
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

macro_rules! set_bench {
    ($bench_name: ident, $sets: expr, $consume_name: ident, $oper_name: expr) => {
        #[bench]
        pub fn $bench_name(b: &mut test::Bencher) {
            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = $oper_name(&sets[0], &sets[1]).$consume_name();
                test::black_box(x);
            })
        }
    };
}

macro_rules! actual_bench {
    ($bench_name: ident, $sets: expr, $consume_name: ident, $oper_name: ident) => {
        set_bench!(
            $bench_name,
            $sets,
            $consume_name,
            rust_bench_btreeset_intersection::set_now::$oper_name
        );
    };
}

macro_rules! future_bench {
    ($bench_name: ident, $sets: expr, $consume_name: ident, $oper_name: ident) => {
        set_bench!(
            $bench_name,
            $sets,
            $consume_name,
            rust_bench_btreeset_intersection::set::$oper_name
        );
    };
}

#[cfg(feature = "diff")]
mod difference_neg_vs_pos {
    use super::{neg, pos};
    actual_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)], count, difference}
    future_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], count, difference_future}
    actual_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)], count, difference}
    future_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], count, difference_future}
    actual_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)], count, difference}
    future_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], count, difference_future}
    actual_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)], count, difference}
    future_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], count, difference_future}
    actual_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)], count, difference}
    future_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], count, difference_future}
    actual_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)], count, difference}
    future_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], count, difference_future}
    actual_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)], count, difference}
    future_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], count, difference_future}
    actual_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)], count, difference}
    future_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], count, difference_future}
}

#[cfg(feature = "diff")]
mod difference_subsets {
    use super::subsets;
    actual_bench! {_10_vs_100,         subsets(10, 10), count, difference}
    future_bench! {_10_vs_100_future,  subsets(10, 10), count, difference_future}
    actual_bench! {_100_vs_10k,        subsets(100, 100), count, difference}
    future_bench! {_100_vs_10k_future, subsets(100, 100), count, difference_future}
}

#[cfg(feature = "diff")]
mod is_subset_neg_vs_pos {
    use super::{neg, pos};
    actual_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)], clone, is_subset}
    future_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], clone, is_subset_future}
    actual_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)], clone, is_subset}
    future_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], clone, is_subset_future}
    actual_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)], clone, is_subset}
    future_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], clone, is_subset_future}
    actual_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)], clone, is_subset}
    future_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], clone, is_subset_future}
    actual_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)], clone, is_subset}
    future_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], clone, is_subset_future}
    actual_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)], clone, is_subset}
    future_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], clone, is_subset_future}
    actual_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)], clone, is_subset}
    future_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], clone, is_subset_future}
    actual_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)], clone, is_subset}
    future_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], clone, is_subset_future}
}

#[cfg(feature = "diff")]
mod is_subset_subsets {
    use super::subsets;
    actual_bench! {_10_vs_100,         subsets(10, 10), clone, is_subset}
    future_bench! {_10_vs_100_future,  subsets(10, 10), clone, is_subset_future}
    actual_bench! {_100_vs_10k,        subsets(100, 100), clone, is_subset}
    future_bench! {_100_vs_10k_future, subsets(100, 100), clone, is_subset_future}
}

#[cfg(feature = "intersect")]
mod intersect_neg_vs_pos {
    use super::{neg, pos};
    actual_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)], count, intersection}
    future_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], count, intersection_future}
    future_bench! {_100_neg_vs_100_pos_switch, [neg(100), pos(100)], count, intersection_switch}
    future_bench! {_100_neg_vs_100_pos_swivel, [neg(100), pos(100)], count, intersection_swivel}
    actual_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)], count, intersection}
    future_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], count, intersection_future}
    future_bench! {_100_neg_vs_10k_pos_switch, [neg(100), pos(10_000)], count, intersection_switch}
    future_bench! {_100_neg_vs_10k_pos_swivel, [neg(100), pos(10_000)], count, intersection_swivel}
    actual_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)], count, intersection}
    future_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], count, intersection_future}
    future_bench! {_100_pos_vs_100_neg_switch, [pos(100), neg(100)], count, intersection_switch}
    future_bench! {_100_pos_vs_100_neg_swivel, [pos(100), neg(100)], count, intersection_swivel}
    actual_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)], count, intersection}
    future_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], count, intersection_future}
    future_bench! {_100_pos_vs_10k_neg_switch, [pos(100), neg(10_000)], count, intersection_switch}
    future_bench! {_100_pos_vs_10k_neg_swivel, [pos(100), neg(10_000)], count, intersection_swivel}
    actual_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)], count, intersection}
    future_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], count, intersection_future}
    future_bench! {_10k_neg_vs_100_pos_switch, [neg(10_000), pos(100)], count, intersection_switch}
    future_bench! {_10k_neg_vs_100_pos_swivel, [neg(10_000), pos(100)], count, intersection_swivel}
    actual_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)], count, intersection}
    future_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], count, intersection_future}
    future_bench! {_10k_neg_vs_10k_pos_switch, [neg(10_000), pos(10_000)], count, intersection_switch}
    future_bench! {_10k_neg_vs_10k_pos_swivel, [neg(10_000), pos(10_000)], count, intersection_swivel}
    actual_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)], count, intersection}
    future_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], count, intersection_future}
    future_bench! {_10k_pos_vs_100_neg_switch, [pos(10_000), neg(100)], count, intersection_switch}
    future_bench! {_10k_pos_vs_100_neg_swivel, [pos(10_000), neg(100)], count, intersection_swivel}
    actual_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)], count, intersection}
    future_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], count, intersection_future}
    future_bench! {_10k_pos_vs_10k_neg_switch, [pos(10_000), neg(10_000)], count, intersection_switch}
    future_bench! {_10k_pos_vs_10k_neg_swivel, [pos(10_000), neg(10_000)], count, intersection_swivel}
}

#[cfg(feature = "intersect")]
mod intersection_subsets {
    use super::subsets;
    actual_bench! {_10_vs_100,         subsets(10, 10), count, intersection}
    future_bench! {_10_vs_100_future,  subsets(10, 10), count, intersection_future}
    future_bench! {_10_vs_100_switch,  subsets(10, 10), count, intersection_switch}
    future_bench! {_10_vs_100_swivel,  subsets(10, 10), count, intersection_swivel}
    actual_bench! {_100_vs_10k,        subsets(100, 100), count, intersection}
    future_bench! {_100_vs_10k_future, subsets(100, 100), count, intersection_future}
    future_bench! {_100_vs_10k_switch, subsets(100, 100), count, intersection_switch}
    future_bench! {_100_vs_10k_swivel, subsets(100, 100), count, intersection_swivel}
}

#[cfg(feature = "merge")]
mod symmdiff_neg_vs_pos {
    use super::{neg, pos};
    actual_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)], count, symmetric_difference}
    future_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], count, symmdiff_future}
    actual_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)], count, symmetric_difference}
    future_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], count, symmdiff_future}
    actual_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)], count, symmetric_difference}
    future_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], count, symmdiff_future}
    actual_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)], count, symmetric_difference}
    future_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], count, symmdiff_future}
    actual_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)], count, symmetric_difference}
    future_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], count, symmdiff_future}
    actual_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)], count, symmetric_difference}
    future_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], count, symmdiff_future}
    actual_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)], count, symmetric_difference}
    future_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], count, symmdiff_future}
    actual_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)], count, symmetric_difference}
    future_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], count, symmdiff_future}
}

#[cfg(feature = "merge")]
mod symmdiff_subsets {
    use super::subsets;
    actual_bench! {_10_vs_100,         subsets(10, 10), count, symmetric_difference}
    future_bench! {_10_vs_100_future,  subsets(10, 10), count, symmdiff_future}
    actual_bench! {_100_vs_10k,        subsets(100, 100), count, symmetric_difference}
    future_bench! {_100_vs_10k_future, subsets(100, 100), count, symmdiff_future}
}

#[cfg(feature = "merge")]
mod union_neg_vs_pos {
    use super::{neg, pos};
    actual_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)], count, union}
    future_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], count, union_future}
    actual_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)], count, union}
    future_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], count, union_future}
    actual_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)], count, union}
    future_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], count, union_future}
    actual_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)], count, union}
    future_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], count, union_future}
    actual_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)], count, union}
    future_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], count, union_future}
    actual_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)], count, union}
    future_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], count, union_future}
    actual_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)], count, union}
    future_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], count, union_future}
    actual_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)], count, union}
    future_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], count, union_future}
}

#[cfg(feature = "merge")]
mod union_subsets {
    use super::subsets;
    actual_bench! {_10_vs_100,         subsets(10, 10), count, union}
    future_bench! {_10_vs_100_future,  subsets(10, 10), count, union_future}
    actual_bench! {_100_vs_10k,        subsets(100, 100), count, union}
    future_bench! {_100_vs_10k_future, subsets(100, 100), count, union_future}
}

#[cfg(feature = "diff")]
mod difference_random_100 {
    use super::random;
    actual_bench! {vs_100,            random(100, 100), count, difference}
    future_bench! {vs_100_future,     random(100, 100), count, difference_future}
    actual_bench! {vs_1600,           random(100, 1_600), count, difference}
    future_bench! {vs_1600_future,    random(100, 1_600), count, difference_future}
    actual_bench! {vs_10k,            random(100, 10_000), count, difference}
    future_bench! {vs_10k_future,     random(100, 10_000), count, difference_future}
}

#[cfg(feature = "diff")]
mod is_subset_random_100 {
    use super::random;
    actual_bench! {vs_100,            random(100, 100), clone, is_subset}
    future_bench! {vs_100_future,     random(100, 100), clone, is_subset_future}
    actual_bench! {vs_1600,           random(100, 1_600), clone, is_subset}
    future_bench! {vs_1600_future,    random(100, 1_600), clone, is_subset_future}
    actual_bench! {vs_10k,            random(100, 10_000), clone, is_subset}
    future_bench! {vs_10k_future,     random(100, 10_000), clone, is_subset_future}
}

#[cfg(feature = "intersect")]
mod intersect_random_100 {
    use super::random;
    actual_bench! {vs_100,            random(100, 100), count, intersection}
    future_bench! {vs_100_future,     random(100, 100), count, intersection_future}
    future_bench! {vs_100_search,     random(100, 100), count, intersection_search}
    future_bench! {vs_100_stitch,     random(100, 100), count, intersection_stitch}
    future_bench! {vs_100_switch,     random(100, 100), count, intersection_switch}
    future_bench! {vs_100_swivel,     random(100, 100), count, intersection_swivel}
    actual_bench! {vs_1600,           random(100, 1_600), count, intersection}
    future_bench! {vs_1600_future,    random(100, 1_600), count, intersection_future}
    future_bench! {vs_1600_search,    random(100, 1_600), count, intersection_search}
    future_bench! {vs_1600_stitch,    random(100, 1_600), count, intersection_stitch}
    future_bench! {vs_1600_switch,    random(100, 1_600), count, intersection_switch}
    future_bench! {vs_1600_swivel,    random(100, 1_600), count, intersection_swivel}
    actual_bench! {vs_10k,            random(100, 10_000), count, intersection}
    future_bench! {vs_10k_future,     random(100, 10_000), count, intersection_future}
    future_bench! {vs_10k_search,     random(100, 10_000), count, intersection_search}
    future_bench! {vs_10k_stitch,     random(100, 10_000), count, intersection_stitch}
    future_bench! {vs_10k_switch,     random(100, 10_000), count, intersection_switch}
    future_bench! {vs_10k_swivel,     random(100, 10_000), count, intersection_swivel}
}

#[cfg(feature = "merge")]
mod symmdiff_random_100 {
    use super::random;
    actual_bench! {vs_100,            random(100, 100), count, symmetric_difference}
    future_bench! {vs_100_future,     random(100, 100), count, symmdiff_future}
    actual_bench! {vs_1600,           random(100, 1_600), count, symmetric_difference}
    future_bench! {vs_1600_future,    random(100, 1_600), count, symmdiff_future}
    actual_bench! {vs_10k,            random(100, 10_000), count, symmetric_difference}
    future_bench! {vs_10k_future,     random(100, 10_000), count, symmdiff_future}
}

#[cfg(feature = "merge")]
mod union_random_100 {
    use super::random;
    actual_bench! {vs_100,            random(100, 100), count, union}
    future_bench! {vs_100_future,     random(100, 100), count, union_future}
    actual_bench! {vs_1600,           random(100, 1_600), count, union}
    future_bench! {vs_1600_future,    random(100, 1_600), count, union_future}
    actual_bench! {vs_10k,            random(100, 10_000), count, union}
    future_bench! {vs_10k_future,     random(100, 10_000), count, union_future}
}

#[cfg(feature = "diff")]
mod difference_random_10k {
    use super::random;
    actual_bench! {vs_10k,            random(10_000, 10_000), count, difference}
    future_bench! {vs_10k_future,     random(10_000, 10_000), count, difference_future}
    #[cfg(feature = "include_100k")]
    actual_bench! {vs_160k,           random(10_000, 160_000), count, difference}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_future,    random(10_000, 160_000), count, difference_future}
}

#[cfg(feature = "diff")]
mod is_subset_random_10k {
    use super::random;
    actual_bench! {vs_10k,            random(10_000, 10_000), clone, is_subset}
    future_bench! {vs_10k_future,     random(10_000, 10_000), clone, is_subset_future}
    #[cfg(feature = "include_100k")]
    actual_bench! {vs_160k,           random(10_000, 160_000), clone, is_subset}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_future,    random(10_000, 160_000), clone, is_subset_future}
}

#[cfg(feature = "intersect")]
mod intersect_random_10k {
    use super::random;
    actual_bench! {vs_10k,            random(10_000, 10_000), count, intersection}
    future_bench! {vs_10k_future,     random(10_000, 10_000), count, intersection_future}
    future_bench! {vs_10k_search,     random(10_000, 10_000), count, intersection_search}
    future_bench! {vs_10k_stitch,     random(10_000, 10_000), count, intersection_stitch}
    future_bench! {vs_10k_switch,     random(10_000, 10_000), count, intersection_switch}
    future_bench! {vs_10k_swivel,     random(10_000, 10_000), count, intersection_swivel}
    #[cfg(feature = "include_100k")]
    actual_bench! {vs_160k,           random(10_000, 160_000), count, intersection}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_future,    random(10_000, 160_000), count, intersection_future}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_search,    random(10_000, 160_000), count, intersection_search}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_stitch,    random(10_000, 160_000), count, intersection_stitch}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_switch,    random(10_000, 160_000), count, intersection_switch}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_swivel,    random(10_000, 160_000), count, intersection_swivel}
}

#[cfg(feature = "merge")]
mod symmdiff_random_10k {
    use super::random;
    actual_bench! {vs_10k,            random(10_000, 10_000), count, symmetric_difference}
    future_bench! {vs_10k_future,     random(10_000, 10_000), count, symmdiff_future}
    #[cfg(feature = "include_100k")]
    actual_bench! {vs_160k,           random(10_000, 160_000), count, symmetric_difference}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_future,    random(10_000, 160_000), count, symmdiff_future}
}

#[cfg(feature = "merge")]
mod union_random_10k {
    use super::random;
    actual_bench! {vs_10k,            random(10_000, 10_000), count, union}
    future_bench! {vs_10k_future,     random(10_000, 10_000), count, union_future}
    #[cfg(feature = "include_100k")]
    actual_bench! {vs_160k,           random(10_000, 160_000), count, union}
    #[cfg(feature = "include_100k")]
    future_bench! {vs_160k_future,    random(10_000, 160_000), count, union_future}
}

#[cfg(feature = "stagger")]
mod stagger_000_001 {
    use super::stagger;
    actual_bench! {vs_1,          stagger(1, 1), count, intersection}
    future_bench! {vs_1_future,   stagger(1, 1), count, intersection_future}
    future_bench! {vs_1_search,   stagger(1, 1), count, intersection_search}
    future_bench! {vs_1_stitch,   stagger(1, 1), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_002 {
    use super::stagger;
    actual_bench! {vs_2,          stagger(2, 1), count, intersection}
    future_bench! {vs_2_future,   stagger(2, 1), count, intersection_future}
    future_bench! {vs_2_search,   stagger(2, 1), count, intersection_search}
    future_bench! {vs_2_stitch,   stagger(2, 1), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_004 {
    use super::stagger;
    actual_bench! {vs_4,          stagger(4, 1), count, intersection}
    future_bench! {vs_4_future,   stagger(4, 1), count, intersection_future}
    future_bench! {vs_4_search,   stagger(4, 1), count, intersection_search}
    future_bench! {vs_4_stitch,   stagger(4, 1), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_006 {
    use super::stagger;
    actual_bench! {vs_6,          stagger(6, 1), count, intersection}
    future_bench! {vs_6_future,   stagger(6, 1), count, intersection_future}
    future_bench! {vs_6_search,   stagger(6, 1), count, intersection_search}
    future_bench! {vs_6_stitch,   stagger(6, 1), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_008 {
    use super::stagger;
    actual_bench! {vs_8,          stagger(8, 1), count, intersection}
    future_bench! {vs_8_future,   stagger(8, 1), count, intersection_future}
    future_bench! {vs_8_search,   stagger(8, 1), count, intersection_search}
    future_bench! {vs_8_stitch,   stagger(8, 1), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_010 {
    use super::stagger;
    actual_bench! {vs_x02,        stagger(10, 2), count, intersection}
    future_bench! {vs_x02_future, stagger(10, 2), count, intersection_future}
    future_bench! {vs_x02_search, stagger(10, 2), count, intersection_search}
    future_bench! {vs_x02_stitch, stagger(10, 2), count, intersection_stitch}
    actual_bench! {vs_x03,        stagger(10, 3), count, intersection}
    future_bench! {vs_x03_future, stagger(10, 3), count, intersection_future}
    future_bench! {vs_x03_search, stagger(10, 3), count, intersection_search}
    future_bench! {vs_x03_stitch, stagger(10, 3), count, intersection_stitch}
    actual_bench! {vs_x04,        stagger(10, 4), count, intersection}
    future_bench! {vs_x04_future, stagger(10, 4), count, intersection_future}
    future_bench! {vs_x04_search, stagger(10, 4), count, intersection_search}
    future_bench! {vs_x04_stitch, stagger(10, 4), count, intersection_stitch}
    actual_bench! {vs_x05,        stagger(10, 5), count, intersection}
    future_bench! {vs_x05_future, stagger(10, 5), count, intersection_future}
    future_bench! {vs_x05_search, stagger(10, 5), count, intersection_search}
    future_bench! {vs_x05_stitch, stagger(10, 5), count, intersection_stitch}
    actual_bench! {vs_x15,        stagger(10, 15), count, intersection}
    future_bench! {vs_x15_future, stagger(10, 15), count, intersection_future}
    future_bench! {vs_x15_search, stagger(10, 15), count, intersection_search}
    future_bench! {vs_x15_stitch, stagger(10, 15), count, intersection_stitch}
    actual_bench! {vs_x16,        stagger(10, 16), count, intersection}
    future_bench! {vs_x16_future, stagger(10, 16), count, intersection_future}
    future_bench! {vs_x16_search, stagger(10, 16), count, intersection_search}
    future_bench! {vs_x16_stitch, stagger(10, 16), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_100 {
    use super::stagger;
    actual_bench! {vs_x04,        stagger(100, 4), count, intersection}
    future_bench! {vs_x04_future, stagger(100, 4), count, intersection_future}
    future_bench! {vs_x04_search, stagger(100, 4), count, intersection_search}
    future_bench! {vs_x04_stitch, stagger(100, 4), count, intersection_stitch}
    actual_bench! {vs_x05,        stagger(100, 5), count, intersection}
    future_bench! {vs_x05_future, stagger(100, 5), count, intersection_future}
    future_bench! {vs_x05_search, stagger(100, 5), count, intersection_search}
    future_bench! {vs_x05_stitch, stagger(100, 5), count, intersection_stitch}
    actual_bench! {vs_x06,        stagger(100, 6), count, intersection}
    future_bench! {vs_x06_future, stagger(100, 6), count, intersection_future}
    future_bench! {vs_x06_search, stagger(100, 6), count, intersection_search}
    future_bench! {vs_x06_stitch, stagger(100, 6), count, intersection_stitch}
    actual_bench! {vs_x07,        stagger(100, 7), count, intersection}
    future_bench! {vs_x07_future, stagger(100, 7), count, intersection_future}
    future_bench! {vs_x07_search, stagger(100, 7), count, intersection_search}
    future_bench! {vs_x07_stitch, stagger(100, 7), count, intersection_stitch}
    actual_bench! {vs_x15,        stagger(100, 15), count, intersection}
    future_bench! {vs_x15_future, stagger(100, 15), count, intersection_future}
    future_bench! {vs_x15_search, stagger(100, 15), count, intersection_search}
    future_bench! {vs_x15_stitch, stagger(100, 15), count, intersection_stitch}
    actual_bench! {vs_x16,        stagger(100, 16), count, intersection}
    future_bench! {vs_x16_future, stagger(100, 16), count, intersection_future}
    future_bench! {vs_x16_search, stagger(100, 16), count, intersection_search}
    future_bench! {vs_x16_stitch, stagger(100, 16), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_200 {
    use super::stagger;
    actual_bench! {vs_x05,        stagger(200, 5), count, intersection}
    future_bench! {vs_x05_future, stagger(200, 5), count, intersection_future}
    future_bench! {vs_x05_search, stagger(200, 5), count, intersection_search}
    future_bench! {vs_x05_stitch, stagger(200, 5), count, intersection_stitch}
    actual_bench! {vs_x06,        stagger(200, 6), count, intersection}
    future_bench! {vs_x06_future, stagger(200, 6), count, intersection_future}
    future_bench! {vs_x06_search, stagger(200, 6), count, intersection_search}
    future_bench! {vs_x06_stitch, stagger(200, 6), count, intersection_stitch}
    actual_bench! {vs_x07,        stagger(200, 7), count, intersection}
    future_bench! {vs_x07_future, stagger(200, 7), count, intersection_future}
    future_bench! {vs_x07_search, stagger(200, 7), count, intersection_search}
    future_bench! {vs_x07_stitch, stagger(200, 7), count, intersection_stitch}
    actual_bench! {vs_x08,        stagger(200, 8), count, intersection}
    future_bench! {vs_x08_future, stagger(200, 8), count, intersection_future}
    future_bench! {vs_x08_search, stagger(200, 8), count, intersection_search}
    future_bench! {vs_x08_stitch, stagger(200, 8), count, intersection_stitch}
    actual_bench! {vs_x15,        stagger(200, 15), count, intersection}
    future_bench! {vs_x15_future, stagger(200, 15), count, intersection_future}
    future_bench! {vs_x15_search, stagger(200, 15), count, intersection_search}
    future_bench! {vs_x15_stitch, stagger(200, 15), count, intersection_stitch}
    actual_bench! {vs_x16,        stagger(200, 16), count, intersection}
    future_bench! {vs_x16_future, stagger(200, 16), count, intersection_future}
    future_bench! {vs_x16_search, stagger(200, 16), count, intersection_search}
    future_bench! {vs_x16_stitch, stagger(200, 16), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_000_500 {
    use super::stagger;
    actual_bench! {vs_x12,        stagger(500, 12), count, intersection}
    future_bench! {vs_x12_future, stagger(500, 12), count, intersection_future}
    future_bench! {vs_x12_search, stagger(500, 12), count, intersection_search}
    future_bench! {vs_x12_stitch, stagger(500, 12), count, intersection_stitch}
    actual_bench! {vs_x13,        stagger(500, 13), count, intersection}
    future_bench! {vs_x13_future, stagger(500, 13), count, intersection_future}
    future_bench! {vs_x13_search, stagger(500, 13), count, intersection_search}
    future_bench! {vs_x13_stitch, stagger(500, 13), count, intersection_stitch}
    actual_bench! {vs_x14,        stagger(500, 14), count, intersection}
    future_bench! {vs_x14_future, stagger(500, 14), count, intersection_future}
    future_bench! {vs_x14_search, stagger(500, 14), count, intersection_search}
    future_bench! {vs_x14_stitch, stagger(500, 14), count, intersection_stitch}
    actual_bench! {vs_x15,        stagger(500, 15), count, intersection}
    future_bench! {vs_x15_future, stagger(500, 15), count, intersection_future}
    future_bench! {vs_x15_search, stagger(500, 15), count, intersection_search}
    future_bench! {vs_x15_stitch, stagger(500, 15), count, intersection_stitch}
    actual_bench! {vs_x16,        stagger(500, 16), count, intersection}
    future_bench! {vs_x16_future, stagger(500, 16), count, intersection_future}
    future_bench! {vs_x16_search, stagger(500, 16), count, intersection_search}
    future_bench! {vs_x16_stitch, stagger(500, 16), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_001_000 {
    use super::stagger;
    actual_bench! {vs_x15,        stagger(1_000, 15), count, intersection}
    future_bench! {vs_x15_future, stagger(1_000, 15), count, intersection_future}
    future_bench! {vs_x15_search, stagger(1_000, 15), count, intersection_search}
    future_bench! {vs_x15_stitch, stagger(1_000, 15), count, intersection_stitch}
    actual_bench! {vs_x16,        stagger(1_000, 16), count, intersection}
    future_bench! {vs_x16_future, stagger(1_000, 16), count, intersection_future}
    future_bench! {vs_x16_search, stagger(1_000, 16), count, intersection_search}
    future_bench! {vs_x16_stitch, stagger(1_000, 16), count, intersection_stitch}
    actual_bench! {vs_x17,        stagger(1_000, 17), count, intersection}
    future_bench! {vs_x17_future, stagger(1_000, 17), count, intersection_future}
    future_bench! {vs_x17_search, stagger(1_000, 17), count, intersection_search}
    future_bench! {vs_x17_stitch, stagger(1_000, 17), count, intersection_stitch}
    actual_bench! {vs_x18,        stagger(1_000, 18), count, intersection}
    future_bench! {vs_x18_future, stagger(1_000, 18), count, intersection_future}
    future_bench! {vs_x18_search, stagger(1_000, 18), count, intersection_search}
    future_bench! {vs_x18_stitch, stagger(1_000, 18), count, intersection_stitch}
    actual_bench! {vs_x19,        stagger(1_000, 19), count, intersection}
    future_bench! {vs_x19_future, stagger(1_000, 19), count, intersection_future}
    future_bench! {vs_x19_search, stagger(1_000, 19), count, intersection_search}
    future_bench! {vs_x19_stitch, stagger(1_000, 19), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_010_000 {
    use super::stagger;
    actual_bench! {vs_x15,        stagger(10_000, 15), count, intersection}
    future_bench! {vs_x15_future, stagger(10_000, 15), count, intersection_future}
    future_bench! {vs_x15_search, stagger(10_000, 15), count, intersection_search}
    future_bench! {vs_x15_stitch, stagger(10_000, 15), count, intersection_stitch}
    actual_bench! {vs_x16,        stagger(10_000, 16), count, intersection}
    future_bench! {vs_x16_future, stagger(10_000, 16), count, intersection_future}
    future_bench! {vs_x16_search, stagger(10_000, 16), count, intersection_search}
    future_bench! {vs_x16_stitch, stagger(10_000, 16), count, intersection_stitch}
    actual_bench! {vs_x17,        stagger(10_000, 17), count, intersection}
    future_bench! {vs_x17_future, stagger(10_000, 17), count, intersection_future}
    future_bench! {vs_x17_search, stagger(10_000, 17), count, intersection_search}
    future_bench! {vs_x17_stitch, stagger(10_000, 17), count, intersection_stitch}
    actual_bench! {vs_x18,        stagger(10_000, 18), count, intersection}
    future_bench! {vs_x18_future, stagger(10_000, 18), count, intersection_future}
    future_bench! {vs_x18_search, stagger(10_000, 18), count, intersection_search}
    future_bench! {vs_x18_stitch, stagger(10_000, 18), count, intersection_stitch}
    actual_bench! {vs_x19,        stagger(10_000, 19), count, intersection}
    future_bench! {vs_x19_future, stagger(10_000, 19), count, intersection_future}
    future_bench! {vs_x19_search, stagger(10_000, 19), count, intersection_search}
    future_bench! {vs_x19_stitch, stagger(10_000, 19), count, intersection_stitch}
    actual_bench! {vs_x20,        stagger(10_000, 20), count, intersection}
    future_bench! {vs_x20_future, stagger(10_000, 20), count, intersection_future}
    future_bench! {vs_x20_search, stagger(10_000, 20), count, intersection_search}
    future_bench! {vs_x20_stitch, stagger(10_000, 20), count, intersection_stitch}
}

#[cfg(feature = "stagger")]
mod stagger_100_000 {
    use super::stagger;
    actual_bench! {vs_x15,        stagger(100_000, 15), count, intersection}
    future_bench! {vs_x15_future, stagger(100_000, 15), count, intersection_future}
    future_bench! {vs_x15_search, stagger(100_000, 15), count, intersection_search}
    future_bench! {vs_x15_stitch, stagger(100_000, 15), count, intersection_stitch}
    actual_bench! {vs_x16,        stagger(100_000, 16), count, intersection}
    future_bench! {vs_x16_future, stagger(100_000, 16), count, intersection_future}
    future_bench! {vs_x16_search, stagger(100_000, 16), count, intersection_search}
    future_bench! {vs_x16_stitch, stagger(100_000, 16), count, intersection_stitch}
    actual_bench! {vs_x17,        stagger(100_000, 17), count, intersection}
    future_bench! {vs_x17_future, stagger(100_000, 17), count, intersection_future}
    future_bench! {vs_x17_search, stagger(100_000, 17), count, intersection_search}
    future_bench! {vs_x17_stitch, stagger(100_000, 17), count, intersection_stitch}
    actual_bench! {vs_x18,        stagger(100_000, 18), count, intersection}
    future_bench! {vs_x18_future, stagger(100_000, 18), count, intersection_future}
    future_bench! {vs_x18_search, stagger(100_000, 18), count, intersection_search}
    future_bench! {vs_x18_stitch, stagger(100_000, 18), count, intersection_stitch}
    actual_bench! {vs_x19,        stagger(100_000, 19), count, intersection}
    future_bench! {vs_x19_future, stagger(100_000, 19), count, intersection_future}
    future_bench! {vs_x19_search, stagger(100_000, 19), count, intersection_search}
    future_bench! {vs_x19_stitch, stagger(100_000, 19), count, intersection_stitch}
    actual_bench! {vs_x20,        stagger(100_000, 20), count, intersection}
    future_bench! {vs_x20_future, stagger(100_000, 20), count, intersection_future}
    future_bench! {vs_x20_search, stagger(100_000, 20), count, intersection_search}
    future_bench! {vs_x20_stitch, stagger(100_000, 20), count, intersection_stitch}
}
