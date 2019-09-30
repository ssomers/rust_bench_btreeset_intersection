// file comparable to rust/src/liballoc/benches/btree/set.rs
// Or it used to be.
#![feature(test)]

extern crate rand;
extern crate rand_xorshift;
extern crate test;
use self::rand::{Rng, SeedableRng};
use self::rand_xorshift::XorShiftRng;
use std::cmp::min;
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

fn stagger(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
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

macro_rules! difference_bench {
    ($name: ident, $sets: expr) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = sets[0].difference(&sets[1]).count();
                test::black_box(x);
            })
        }
    };
    ($name: ident, $sets: expr, $difference_kind: ident) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
            use ::rust_bench_btreeset_intersection::set::$difference_kind;

            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = $difference_kind(&sets[0], &sets[1]).count();
                test::black_box(x);
            })
        }
    };
}

macro_rules! is_subset_bench {
    ($name: ident, $sets: expr) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = sets[0].is_subset(&sets[1]);
                test::black_box(x);
            })
        }
    };
    ($name: ident, $sets: expr, $is_subset_kind: ident) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
            use ::rust_bench_btreeset_intersection::set::$is_subset_kind;

            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = $is_subset_kind(&sets[0], &sets[1]);
                test::black_box(x);
            })
        }
    };
}

macro_rules! intersection_bench {
    ($name: ident, $sets: expr) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = sets[0].intersection(&sets[1]).count();
                test::black_box(x);
            })
        }
    };
    ($name: ident, $sets: expr, $intersection_kind: ident) => {
        #[bench]
        pub fn $name(b: &mut test::Bencher) {
            use ::rust_bench_btreeset_intersection::set::$intersection_kind;

            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = $intersection_kind(&sets[0], &sets[1]).count();
                test::black_box(x);
            })
        }
    };
}

mod difference_neg_vs_pos {
    use super::{neg, pos};
    difference_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)]}
    difference_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], difference_future}
    difference_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)]}
    difference_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], difference_future}
    difference_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)]}
    difference_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], difference_future}
    difference_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)]}
    difference_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], difference_future}
    difference_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)]}
    difference_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], difference_future}
    difference_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)]}
    difference_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], difference_future}
    difference_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)]}
    difference_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], difference_future}
    difference_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)]}
    difference_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], difference_future}
}

mod is_subset_neg_vs_pos {
    use super::{neg, pos};
    is_subset_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)]}
    is_subset_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], is_subset_future}
    is_subset_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)]}
    is_subset_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], is_subset_future}
    is_subset_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)]}
    is_subset_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], is_subset_future}
    is_subset_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)]}
    is_subset_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], is_subset_future}
    is_subset_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)]}
    is_subset_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], is_subset_future}
    is_subset_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)]}
    is_subset_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], is_subset_future}
    is_subset_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)]}
    is_subset_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], is_subset_future}
    is_subset_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)]}
    is_subset_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], is_subset_future}
}

mod intersect_neg_vs_pos {
    use super::{neg, pos};
    intersection_bench! {_100_neg_vs_100_pos,        [neg(100), pos(100)]}
    intersection_bench! {_100_neg_vs_100_pos_future, [neg(100), pos(100)], intersection_future}
    intersection_bench! {_100_neg_vs_100_pos_swivel, [neg(100), pos(100)], intersection_swivel}
    intersection_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)]}
    intersection_bench! {_100_neg_vs_10k_pos_future, [neg(100), pos(10_000)], intersection_future}
    intersection_bench! {_100_neg_vs_10k_pos_swivel, [neg(100), pos(10_000)], intersection_swivel}
    intersection_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)]}
    intersection_bench! {_100_pos_vs_100_neg_future, [pos(100), neg(100)], intersection_future}
    intersection_bench! {_100_pos_vs_100_neg_swivel, [pos(100), neg(100)], intersection_swivel}
    intersection_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)]}
    intersection_bench! {_100_pos_vs_10k_neg_future, [pos(100), neg(10_000)], intersection_future}
    intersection_bench! {_100_pos_vs_10k_neg_swivel, [pos(100), neg(10_000)], intersection_swivel}
    intersection_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)]}
    intersection_bench! {_10k_neg_vs_100_pos_future, [neg(10_000), pos(100)], intersection_future}
    intersection_bench! {_10k_neg_vs_100_pos_swivel, [neg(10_000), pos(100)], intersection_swivel}
    intersection_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)]}
    intersection_bench! {_10k_neg_vs_10k_pos_future, [neg(10_000), pos(10_000)], intersection_future}
    intersection_bench! {_10k_neg_vs_10k_pos_swivel, [neg(10_000), pos(10_000)], intersection_swivel}
    intersection_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)]}
    intersection_bench! {_10k_pos_vs_100_neg_future, [pos(10_000), neg(100)], intersection_future}
    intersection_bench! {_10k_pos_vs_100_neg_swivel, [pos(10_000), neg(100)], intersection_swivel}
    intersection_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)]}
    intersection_bench! {_10k_pos_vs_10k_neg_future, [pos(10_000), neg(10_000)], intersection_future}
    intersection_bench! {_10k_pos_vs_10k_neg_swivel, [pos(10_000), neg(10_000)], intersection_swivel}
}

mod difference_random_100 {
    use super::random;
    difference_bench! {vs_100,            random(100, 100)}
    difference_bench! {vs_100_future,     random(100, 100), difference_future}
    difference_bench! {vs_1600,           random(100, 1_600)}
    difference_bench! {vs_1600_future,    random(100, 1_600), difference_future}
    difference_bench! {vs_10k,            random(100, 10_000)}
    difference_bench! {vs_10k_future,     random(100, 10_000), difference_future}
}

mod is_subset_random_100 {
    use super::random;
    is_subset_bench! {vs_100,            random(100, 100)}
    is_subset_bench! {vs_100_future,     random(100, 100), is_subset_future}
    is_subset_bench! {vs_1600,           random(100, 1_600)}
    is_subset_bench! {vs_1600_future,    random(100, 1_600), is_subset_future}
    is_subset_bench! {vs_10k,            random(100, 10_000)}
    is_subset_bench! {vs_10k_future,     random(100, 10_000), is_subset_future}
}

mod intersect_random_100 {
    use super::random;
    intersection_bench! {vs_100,            random(100, 100)}
    intersection_bench! {vs_100_future,     random(100, 100), intersection_future}
    intersection_bench! {vs_100_swivel,     random(100, 100), intersection_swivel}
    intersection_bench! {vs_1600,           random(100, 1_600)}
    intersection_bench! {vs_1600_future,    random(100, 1_600), intersection_future}
    intersection_bench! {vs_1600_swivel,    random(100, 1_600), intersection_swivel}
    intersection_bench! {vs_10k,            random(100, 10_000)}
    intersection_bench! {vs_10k_future,     random(100, 10_000), intersection_future}
    intersection_bench! {vs_10k_swivel,     random(100, 10_000), intersection_swivel}
}

mod difference_random_10k {
    use super::random;
    difference_bench! {vs_10k,            random(10_000, 10_000)}
    difference_bench! {vs_10k_future,     random(10_000, 10_000), difference_future}
    #[cfg(feature = "include_100k")]
    difference_bench! {vs_160k,           random(10_000, 160_000)}
    #[cfg(feature = "include_100k")]
    difference_bench! {vs_160k_future,    random(10_000, 160_000), difference_future}
}

mod is_subset_random_10k {
    use super::random;
    is_subset_bench! {vs_10k,            random(10_000, 10_000)}
    is_subset_bench! {vs_10k_future,     random(10_000, 10_000), is_subset_future}
    #[cfg(feature = "include_100k")]
    is_subset_bench! {vs_160k,           random(10_000, 160_000)}
    #[cfg(feature = "include_100k")]
    is_subset_bench! {vs_160k_future,    random(10_000, 160_000), is_subset_future}
}

mod intersect_random_10k {
    use super::random;
    intersection_bench! {vs_10k,            random(10_000, 10_000)}
    intersection_bench! {vs_10k_future,     random(10_000, 10_000), intersection_future}
    intersection_bench! {vs_10k_swivel,     random(10_000, 10_000), intersection_swivel}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k,           random(10_000, 160_000)}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k_future,    random(10_000, 160_000), intersection_future}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k_swivel,    random(10_000, 160_000), intersection_swivel}
}

mod stagger_000_001 {
    use super::stagger;
    intersection_bench! {vs_1,          stagger(1, 1)}
    intersection_bench! {vs_1_future,   stagger(1, 1), intersection_future}
    intersection_bench! {vs_1_swivel,   stagger(1, 1), intersection_swivel}
}

mod stagger_000_002 {
    use super::stagger;
    intersection_bench! {vs_2,          stagger(2, 1)}
    intersection_bench! {vs_2_future,   stagger(2, 1), intersection_future}
    intersection_bench! {vs_2_swivel,   stagger(2, 1), intersection_swivel}
}

mod stagger_000_004 {
    use super::stagger;
    intersection_bench! {vs_4,          stagger(4, 1)}
    intersection_bench! {vs_4_future,   stagger(4, 1), intersection_future}
    intersection_bench! {vs_4_swivel,   stagger(4, 1), intersection_swivel}
}

mod stagger_000_006 {
    use super::stagger;
    intersection_bench! {vs_6,          stagger(6, 1)}
    intersection_bench! {vs_6_future,   stagger(6, 1), intersection_future}
    intersection_bench! {vs_6_swivel,   stagger(6, 1), intersection_swivel}
}

mod stagger_000_008 {
    use super::stagger;
    intersection_bench! {vs_8,          stagger(8, 1)}
    intersection_bench! {vs_8_future,   stagger(8, 1), intersection_future}
    intersection_bench! {vs_8_swivel,   stagger(8, 1), intersection_swivel}
}

mod stagger_000_010 {
    use super::stagger;
    intersection_bench! {vs_x02,        stagger(10, 2)}
    intersection_bench! {vs_x02_future, stagger(10, 2), intersection_future}
    intersection_bench! {vs_x03,        stagger(10, 3)}
    intersection_bench! {vs_x03_future, stagger(10, 3), intersection_future}
    intersection_bench! {vs_x03_swivel, stagger(10, 3), intersection_swivel}
    intersection_bench! {vs_x04,        stagger(10, 4)}
    intersection_bench! {vs_x04_future, stagger(10, 4), intersection_future}
    intersection_bench! {vs_x04_swivel, stagger(10, 4), intersection_swivel}
    intersection_bench! {vs_x05,        stagger(10, 5)}
    intersection_bench! {vs_x05_future, stagger(10, 5), intersection_future}
    intersection_bench! {vs_x05_swivel, stagger(10, 5), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(10, 15)}
    intersection_bench! {vs_x15_future, stagger(10, 15), intersection_future}
    intersection_bench! {vs_x15_swivel, stagger(10, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(10, 16)}
    intersection_bench! {vs_x16_future, stagger(10, 16), intersection_future}
    intersection_bench! {vs_x16_swivel, stagger(10, 16), intersection_swivel}
}

mod stagger_000_100 {
    use super::stagger;
    intersection_bench! {vs_x04,        stagger(100, 4)}
    intersection_bench! {vs_x04_future, stagger(100, 4), intersection_future}
    intersection_bench! {vs_x04_swivel, stagger(100, 4), intersection_swivel}
    intersection_bench! {vs_x05,        stagger(100, 5)}
    intersection_bench! {vs_x05_future, stagger(100, 5), intersection_future}
    intersection_bench! {vs_x05_swivel, stagger(100, 5), intersection_swivel}
    intersection_bench! {vs_x06,        stagger(100, 6)}
    intersection_bench! {vs_x06_future, stagger(100, 6), intersection_future}
    intersection_bench! {vs_x06_swivel, stagger(100, 6), intersection_swivel}
    intersection_bench! {vs_x07,        stagger(100, 7)}
    intersection_bench! {vs_x07_future, stagger(100, 7), intersection_future}
    intersection_bench! {vs_x07_swivel, stagger(100, 7), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(100, 15)}
    intersection_bench! {vs_x15_future, stagger(100, 15), intersection_future}
    intersection_bench! {vs_x15_swivel, stagger(100, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(100, 16)}
    intersection_bench! {vs_x16_future, stagger(100, 16), intersection_future}
    intersection_bench! {vs_x16_swivel, stagger(100, 16), intersection_swivel}
}

mod stagger_000_200 {
    use super::stagger;
    intersection_bench! {vs_x05,        stagger(200, 5)}
    intersection_bench! {vs_x05_future, stagger(200, 5), intersection_future}
    intersection_bench! {vs_x05_swivel, stagger(200, 5), intersection_swivel}
    intersection_bench! {vs_x06,        stagger(200, 6)}
    intersection_bench! {vs_x06_future, stagger(200, 6), intersection_future}
    intersection_bench! {vs_x06_swivel, stagger(200, 6), intersection_swivel}
    intersection_bench! {vs_x07,        stagger(200, 7)}
    intersection_bench! {vs_x07_future, stagger(200, 7), intersection_future}
    intersection_bench! {vs_x07_swivel, stagger(200, 7), intersection_swivel}
    intersection_bench! {vs_x08,        stagger(200, 8)}
    intersection_bench! {vs_x08_future, stagger(200, 8), intersection_future}
    intersection_bench! {vs_x08_swivel, stagger(200, 8), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(200, 15)}
    intersection_bench! {vs_x15_future, stagger(200, 15), intersection_future}
    intersection_bench! {vs_x15_swivel, stagger(200, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(200, 16)}
    intersection_bench! {vs_x16_future, stagger(200, 16), intersection_future}
    intersection_bench! {vs_x16_swivel, stagger(200, 16), intersection_swivel}
}

mod stagger_000_500 {
    use super::stagger;
    intersection_bench! {vs_x12,        stagger(500, 12)}
    intersection_bench! {vs_x12_future, stagger(500, 12), intersection_future}
    intersection_bench! {vs_x12_swivel, stagger(500, 12), intersection_swivel}
    intersection_bench! {vs_x13,        stagger(500, 13)}
    intersection_bench! {vs_x13_future, stagger(500, 13), intersection_future}
    intersection_bench! {vs_x13_swivel, stagger(500, 13), intersection_swivel}
    intersection_bench! {vs_x14,        stagger(500, 14)}
    intersection_bench! {vs_x14_future, stagger(500, 14), intersection_future}
    intersection_bench! {vs_x14_swivel, stagger(500, 14), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(500, 15)}
    intersection_bench! {vs_x15_future, stagger(500, 15), intersection_future}
    intersection_bench! {vs_x15_swivel, stagger(500, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(500, 16)}
    intersection_bench! {vs_x16_future, stagger(500, 16), intersection_future}
    intersection_bench! {vs_x16_swivel, stagger(500, 16), intersection_swivel}
}

mod stagger_001_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(1_000, 15)}
    intersection_bench! {vs_x15_future, stagger(1_000, 15), intersection_future}
    intersection_bench! {vs_x15_swivel, stagger(1_000, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(1_000, 16)}
    intersection_bench! {vs_x16_future, stagger(1_000, 16), intersection_future}
    intersection_bench! {vs_x16_swivel, stagger(1_000, 16), intersection_swivel}
    intersection_bench! {vs_x17,        stagger(1_000, 17)}
    intersection_bench! {vs_x17_future, stagger(1_000, 17), intersection_future}
    intersection_bench! {vs_x17_swivel, stagger(1_000, 17), intersection_swivel}
    intersection_bench! {vs_x18,        stagger(1_000, 18)}
    intersection_bench! {vs_x18_future, stagger(1_000, 18), intersection_future}
    intersection_bench! {vs_x18_swivel, stagger(1_000, 18), intersection_swivel}
    intersection_bench! {vs_x19,        stagger(1_000, 19)}
    intersection_bench! {vs_x19_future, stagger(1_000, 19), intersection_future}
    intersection_bench! {vs_x19_swivel, stagger(1_000, 19), intersection_swivel}
}

#[cfg(feature = "include_100k")]
mod stagger_010_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(10_000, 15)}
    intersection_bench! {vs_x15_future, stagger(10_000, 15), intersection_future}
    intersection_bench! {vs_x15_swivel, stagger(10_000, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(10_000, 16)}
    intersection_bench! {vs_x16_future, stagger(10_000, 16), intersection_future}
    intersection_bench! {vs_x16_swivel, stagger(10_000, 16), intersection_swivel}
    intersection_bench! {vs_x17,        stagger(10_000, 17)}
    intersection_bench! {vs_x17_future, stagger(10_000, 17), intersection_future}
    intersection_bench! {vs_x17_swivel, stagger(10_000, 17), intersection_swivel}
    intersection_bench! {vs_x18,        stagger(10_000, 18)}
    intersection_bench! {vs_x18_future, stagger(10_000, 18), intersection_future}
    intersection_bench! {vs_x18_swivel, stagger(10_000, 18), intersection_swivel}
    intersection_bench! {vs_x19,        stagger(10_000, 19)}
    intersection_bench! {vs_x19_future, stagger(10_000, 19), intersection_future}
    intersection_bench! {vs_x19_swivel, stagger(10_000, 19), intersection_swivel}
    intersection_bench! {vs_x20,        stagger(10_000, 20)}
    intersection_bench! {vs_x20_future, stagger(10_000, 20), intersection_future}
    intersection_bench! {vs_x20_swivel, stagger(10_000, 20), intersection_swivel}
}

#[cfg(feature = "include_100k")]
mod stagger_100_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(100_000, 15)}
    intersection_bench! {vs_x15_future, stagger(100_000, 15), intersection_future}
    intersection_bench! {vs_x15_swivel, stagger(100_000, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(100_000, 16)}
    intersection_bench! {vs_x16_future, stagger(100_000, 16), intersection_future}
    intersection_bench! {vs_x16_swivel, stagger(100_000, 16), intersection_swivel}
    intersection_bench! {vs_x17,        stagger(100_000, 17)}
    intersection_bench! {vs_x17_future, stagger(100_000, 17), intersection_future}
    intersection_bench! {vs_x17_swivel, stagger(100_000, 17), intersection_swivel}
    intersection_bench! {vs_x18,        stagger(100_000, 18)}
    intersection_bench! {vs_x18_future, stagger(100_000, 18), intersection_future}
    intersection_bench! {vs_x18_swivel, stagger(100_000, 18), intersection_swivel}
    intersection_bench! {vs_x19,        stagger(100_000, 19)}
    intersection_bench! {vs_x19_future, stagger(100_000, 19), intersection_future}
    intersection_bench! {vs_x19_swivel, stagger(100_000, 19), intersection_swivel}
    intersection_bench! {vs_x20,        stagger(100_000, 20)}
    intersection_bench! {vs_x20_future, stagger(100_000, 20), intersection_future}
    intersection_bench! {vs_x20_swivel, stagger(100_000, 20), intersection_swivel}
}
