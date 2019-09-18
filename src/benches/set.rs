// file comparable to rust/src/liballoc/benches/btree/set.rs
#![feature(test)]

extern crate rand;
extern crate test;
use self::rand::{thread_rng, Rng};
use std::cmp::min;
use std::collections::BTreeSet;

fn random(n1: usize, n2: usize) -> [BTreeSet<usize>; 2] {
    let mut rng = thread_rng();
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

mod intersect_random_100 {
    use super::random;
    intersection_bench! {vs_100,            random(100, 100)}
    intersection_bench! {vs_100_future,     random(100, 100), intersection_future}
    intersection_bench! {vs_100_search,     random(100, 100), intersection_search}
    intersection_bench! {vs_100_stitch,     random(100, 100), intersection_stitch}
    intersection_bench! {vs_100_swivel,     random(100, 100), intersection_swivel}
    intersection_bench! {vs_1600,           random(100, 1_600)}
    intersection_bench! {vs_1600_future,    random(100, 1_600), intersection_future}
    intersection_bench! {vs_1600_search,    random(100, 1_600), intersection_search}
    intersection_bench! {vs_1600_stitch,    random(100, 1_600), intersection_stitch}
    intersection_bench! {vs_1600_swivel,    random(100, 1_600), intersection_swivel}
    intersection_bench! {vs_10k,            random(100, 10_000)}
    intersection_bench! {vs_10k_future,     random(100, 10_000), intersection_future}
    intersection_bench! {vs_10k_search,     random(100, 10_000), intersection_search}
    intersection_bench! {vs_10k_stitch,     random(100, 10_000), intersection_stitch}
    intersection_bench! {vs_10k_swivel,     random(100, 10_000), intersection_swivel}
}

mod intersect_random_10k {
    use super::random;
    intersection_bench! {vs_10k,            random(10_000, 10_000)}
    intersection_bench! {vs_10k_future,     random(10_000, 10_000), intersection_future}
    intersection_bench! {vs_10k_search,     random(10_000, 10_000), intersection_search}
    intersection_bench! {vs_10k_stitch,     random(10_000, 10_000), intersection_stitch}
    intersection_bench! {vs_10k_swivel,     random(10_000, 10_000), intersection_swivel}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k,           random(10_000, 160_000)}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k_future,    random(10_000, 160_000), intersection_future}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k_search,    random(10_000, 160_000), intersection_search}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k_swivel,    random(10_000, 160_000), intersection_swivel}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k_stitch,    random(10_000, 160_000), intersection_stitch}
}

mod stagger_000_001 {
    use super::stagger;
    intersection_bench! {vs_1,          stagger(1, 1)}
    intersection_bench! {vs_1_future,   stagger(1, 1), intersection_future}
    intersection_bench! {vs_1_search,   stagger(1, 1), intersection_search}
    intersection_bench! {vs_1_stitch,   stagger(1, 1), intersection_stitch}
    intersection_bench! {vs_1_swivel,   stagger(1, 1), intersection_swivel}
}

mod stagger_000_002 {
    use super::stagger;
    intersection_bench! {vs_2,          stagger(2, 1)}
    intersection_bench! {vs_2_future,   stagger(2, 1), intersection_future}
    intersection_bench! {vs_2_search,   stagger(2, 1), intersection_search}
    intersection_bench! {vs_2_stitch,   stagger(2, 1), intersection_stitch}
    intersection_bench! {vs_2_swivel,   stagger(2, 1), intersection_swivel}
}

mod stagger_000_004 {
    use super::stagger;
    intersection_bench! {vs_4,          stagger(4, 1)}
    intersection_bench! {vs_4_future,   stagger(4, 1), intersection_future}
    intersection_bench! {vs_4_search,   stagger(4, 1), intersection_search}
    intersection_bench! {vs_4_stitch,   stagger(4, 1), intersection_stitch}
    intersection_bench! {vs_4_swivel,   stagger(4, 1), intersection_swivel}
}

mod stagger_000_006 {
    use super::stagger;
    intersection_bench! {vs_6,          stagger(6, 1)}
    intersection_bench! {vs_6_future,   stagger(6, 1), intersection_future}
    intersection_bench! {vs_6_search,   stagger(6, 1), intersection_search}
    intersection_bench! {vs_6_stitch,   stagger(6, 1), intersection_stitch}
    intersection_bench! {vs_6_swivel,   stagger(6, 1), intersection_swivel}
}

mod stagger_000_008 {
    use super::stagger;
    intersection_bench! {vs_8,          stagger(8, 1)}
    intersection_bench! {vs_8_future,   stagger(8, 1), intersection_future}
    intersection_bench! {vs_8_search,   stagger(8, 1), intersection_search}
    intersection_bench! {vs_8_stitch,   stagger(8, 1), intersection_stitch}
    intersection_bench! {vs_8_swivel,   stagger(8, 1), intersection_swivel}
}

mod stagger_000_010 {
    use super::stagger;
    intersection_bench! {vs_x02,        stagger(10, 2)}
    intersection_bench! {vs_x02_future, stagger(10, 2), intersection_future}
    intersection_bench! {vs_x02_search, stagger(10, 2), intersection_search}
    intersection_bench! {vs_x02_stitch, stagger(10, 2), intersection_stitch}
    intersection_bench! {vs_x03,        stagger(10, 3)}
    intersection_bench! {vs_x03_future, stagger(10, 3), intersection_future}
    intersection_bench! {vs_x03_search, stagger(10, 3), intersection_search}
    intersection_bench! {vs_x03_stitch, stagger(10, 3), intersection_stitch}
    intersection_bench! {vs_x03_swivel, stagger(10, 3), intersection_swivel}
    intersection_bench! {vs_x04,        stagger(10, 4)}
    intersection_bench! {vs_x04_future, stagger(10, 4), intersection_future}
    intersection_bench! {vs_x04_search, stagger(10, 4), intersection_search}
    intersection_bench! {vs_x04_stitch, stagger(10, 4), intersection_stitch}
    intersection_bench! {vs_x04_swivel, stagger(10, 4), intersection_swivel}
    intersection_bench! {vs_x05,        stagger(10, 5)}
    intersection_bench! {vs_x05_future, stagger(10, 5), intersection_future}
    intersection_bench! {vs_x05_search, stagger(10, 5), intersection_search}
    intersection_bench! {vs_x05_stitch, stagger(10, 5), intersection_stitch}
    intersection_bench! {vs_x05_swivel, stagger(10, 5), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(10, 15)}
    intersection_bench! {vs_x15_future, stagger(10, 15), intersection_future}
    intersection_bench! {vs_x15_search, stagger(10, 15), intersection_search}
    intersection_bench! {vs_x15_stitch, stagger(10, 15), intersection_stitch}
    intersection_bench! {vs_x15_swivel, stagger(10, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(10, 16)}
    intersection_bench! {vs_x16_future, stagger(10, 16), intersection_future}
    intersection_bench! {vs_x16_search, stagger(10, 16), intersection_search}
    intersection_bench! {vs_x16_stitch, stagger(10, 16), intersection_stitch}
    intersection_bench! {vs_x16_swivel, stagger(10, 16), intersection_swivel}
}

mod stagger_000_100 {
    use super::stagger;
    intersection_bench! {vs_x04,        stagger(100, 4)}
    intersection_bench! {vs_x04_future, stagger(100, 4), intersection_future}
    intersection_bench! {vs_x04_search, stagger(100, 4), intersection_search}
    intersection_bench! {vs_x04_stitch, stagger(100, 4), intersection_stitch}
    intersection_bench! {vs_x04_swivel, stagger(100, 4), intersection_swivel}
    intersection_bench! {vs_x05,        stagger(100, 5)}
    intersection_bench! {vs_x05_future, stagger(100, 5), intersection_future}
    intersection_bench! {vs_x05_search, stagger(100, 5), intersection_search}
    intersection_bench! {vs_x05_stitch, stagger(100, 5), intersection_stitch}
    intersection_bench! {vs_x05_swivel, stagger(100, 5), intersection_swivel}
    intersection_bench! {vs_x06,        stagger(100, 6)}
    intersection_bench! {vs_x06_future, stagger(100, 6), intersection_future}
    intersection_bench! {vs_x06_search, stagger(100, 6), intersection_search}
    intersection_bench! {vs_x06_stitch, stagger(100, 6), intersection_stitch}
    intersection_bench! {vs_x06_swivel, stagger(100, 6), intersection_swivel}
    intersection_bench! {vs_x07,        stagger(100, 7)}
    intersection_bench! {vs_x07_future, stagger(100, 7), intersection_future}
    intersection_bench! {vs_x07_search, stagger(100, 7), intersection_search}
    intersection_bench! {vs_x07_stitch, stagger(100, 7), intersection_stitch}
    intersection_bench! {vs_x07_swivel, stagger(100, 7), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(100, 15)}
    intersection_bench! {vs_x15_future, stagger(100, 15), intersection_future}
    intersection_bench! {vs_x15_search, stagger(100, 15), intersection_search}
    intersection_bench! {vs_x15_stitch, stagger(100, 15), intersection_stitch}
    intersection_bench! {vs_x15_swivel, stagger(100, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(100, 16)}
    intersection_bench! {vs_x16_future, stagger(100, 16), intersection_future}
    intersection_bench! {vs_x16_search, stagger(100, 16), intersection_search}
    intersection_bench! {vs_x16_stitch, stagger(100, 16), intersection_stitch}
    intersection_bench! {vs_x16_swivel, stagger(100, 16), intersection_swivel}
}

mod stagger_000_200 {
    use super::stagger;
    intersection_bench! {vs_x05,        stagger(200, 5)}
    intersection_bench! {vs_x05_future, stagger(200, 5), intersection_future}
    intersection_bench! {vs_x05_search, stagger(200, 5), intersection_search}
    intersection_bench! {vs_x05_stitch, stagger(200, 5), intersection_stitch}
    intersection_bench! {vs_x05_swivel, stagger(200, 5), intersection_swivel}
    intersection_bench! {vs_x06,        stagger(200, 6)}
    intersection_bench! {vs_x06_future, stagger(200, 6), intersection_future}
    intersection_bench! {vs_x06_search, stagger(200, 6), intersection_search}
    intersection_bench! {vs_x06_stitch, stagger(200, 6), intersection_stitch}
    intersection_bench! {vs_x06_swivel, stagger(200, 6), intersection_swivel}
    intersection_bench! {vs_x07,        stagger(200, 7)}
    intersection_bench! {vs_x07_future, stagger(200, 7), intersection_future}
    intersection_bench! {vs_x07_search, stagger(200, 7), intersection_search}
    intersection_bench! {vs_x07_stitch, stagger(200, 7), intersection_stitch}
    intersection_bench! {vs_x07_swivel, stagger(200, 7), intersection_swivel}
    intersection_bench! {vs_x08,        stagger(200, 8)}
    intersection_bench! {vs_x08_future, stagger(200, 8), intersection_future}
    intersection_bench! {vs_x08_search, stagger(200, 8), intersection_search}
    intersection_bench! {vs_x08_stitch, stagger(200, 8), intersection_stitch}
    intersection_bench! {vs_x08_swivel, stagger(200, 8), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(200, 15)}
    intersection_bench! {vs_x15_future, stagger(200, 15), intersection_future}
    intersection_bench! {vs_x15_search, stagger(200, 15), intersection_search}
    intersection_bench! {vs_x15_stitch, stagger(200, 15), intersection_stitch}
    intersection_bench! {vs_x15_swivel, stagger(200, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(200, 16)}
    intersection_bench! {vs_x16_future, stagger(200, 16), intersection_future}
    intersection_bench! {vs_x16_search, stagger(200, 16), intersection_search}
    intersection_bench! {vs_x16_stitch, stagger(200, 16), intersection_stitch}
    intersection_bench! {vs_x16_swivel, stagger(200, 16), intersection_swivel}
}

mod stagger_000_500 {
    use super::stagger;
    intersection_bench! {vs_x12,        stagger(500, 12)}
    intersection_bench! {vs_x12_future, stagger(500, 12), intersection_future}
    intersection_bench! {vs_x12_search, stagger(500, 12), intersection_search}
    intersection_bench! {vs_x12_stitch, stagger(500, 12), intersection_stitch}
    intersection_bench! {vs_x12_swivel, stagger(500, 12), intersection_swivel}
    intersection_bench! {vs_x13,        stagger(500, 13)}
    intersection_bench! {vs_x13_future, stagger(500, 13), intersection_future}
    intersection_bench! {vs_x13_search, stagger(500, 13), intersection_search}
    intersection_bench! {vs_x13_stitch, stagger(500, 13), intersection_stitch}
    intersection_bench! {vs_x13_swivel, stagger(500, 13), intersection_swivel}
    intersection_bench! {vs_x14,        stagger(500, 14)}
    intersection_bench! {vs_x14_future, stagger(500, 14), intersection_future}
    intersection_bench! {vs_x14_search, stagger(500, 14), intersection_search}
    intersection_bench! {vs_x14_stitch, stagger(500, 14), intersection_stitch}
    intersection_bench! {vs_x14_swivel, stagger(500, 14), intersection_swivel}
    intersection_bench! {vs_x15,        stagger(500, 15)}
    intersection_bench! {vs_x15_future, stagger(500, 15), intersection_future}
    intersection_bench! {vs_x15_search, stagger(500, 15), intersection_search}
    intersection_bench! {vs_x15_stitch, stagger(500, 15), intersection_stitch}
    intersection_bench! {vs_x15_swivel, stagger(500, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(500, 16)}
    intersection_bench! {vs_x16_future, stagger(500, 16), intersection_future}
    intersection_bench! {vs_x16_search, stagger(500, 16), intersection_search}
    intersection_bench! {vs_x16_stitch, stagger(500, 16), intersection_stitch}
    intersection_bench! {vs_x16_swivel, stagger(500, 16), intersection_swivel}
}

mod stagger_001_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(1_000, 15)}
    intersection_bench! {vs_x15_future, stagger(1_000, 15), intersection_future}
    intersection_bench! {vs_x15_search, stagger(1_000, 15), intersection_search}
    intersection_bench! {vs_x15_stitch, stagger(1_000, 15), intersection_stitch}
    intersection_bench! {vs_x15_swivel, stagger(1_000, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(1_000, 16)}
    intersection_bench! {vs_x16_future, stagger(1_000, 16), intersection_future}
    intersection_bench! {vs_x16_search, stagger(1_000, 16), intersection_search}
    intersection_bench! {vs_x16_stitch, stagger(1_000, 16), intersection_stitch}
    intersection_bench! {vs_x16_swivel, stagger(1_000, 16), intersection_swivel}
    intersection_bench! {vs_x17,        stagger(1_000, 17)}
    intersection_bench! {vs_x17_future, stagger(1_000, 17), intersection_future}
    intersection_bench! {vs_x17_search, stagger(1_000, 17), intersection_search}
    intersection_bench! {vs_x17_stitch, stagger(1_000, 17), intersection_stitch}
    intersection_bench! {vs_x17_swivel, stagger(1_000, 17), intersection_swivel}
    intersection_bench! {vs_x18,        stagger(1_000, 18)}
    intersection_bench! {vs_x18_future, stagger(1_000, 18), intersection_future}
    intersection_bench! {vs_x18_search, stagger(1_000, 18), intersection_search}
    intersection_bench! {vs_x18_stitch, stagger(1_000, 18), intersection_stitch}
    intersection_bench! {vs_x18_swivel, stagger(1_000, 18), intersection_swivel}
    intersection_bench! {vs_x19,        stagger(1_000, 19)}
    intersection_bench! {vs_x19_future, stagger(1_000, 19), intersection_future}
    intersection_bench! {vs_x19_search, stagger(1_000, 19), intersection_search}
    intersection_bench! {vs_x19_stitch, stagger(1_000, 19), intersection_stitch}
    intersection_bench! {vs_x19_swivel, stagger(1_000, 19), intersection_swivel}
}

#[cfg(feature = "include_100k")]
mod stagger_010_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(10_000, 15)}
    intersection_bench! {vs_x15_future, stagger(10_000, 15), intersection_future}
    intersection_bench! {vs_x15_search, stagger(10_000, 15), intersection_search}
    intersection_bench! {vs_x15_stitch, stagger(10_000, 15), intersection_stitch}
    intersection_bench! {vs_x15_swivel, stagger(10_000, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(10_000, 16)}
    intersection_bench! {vs_x16_future, stagger(10_000, 16), intersection_future}
    intersection_bench! {vs_x16_search, stagger(10_000, 16), intersection_search}
    intersection_bench! {vs_x16_stitch, stagger(10_000, 16), intersection_stitch}
    intersection_bench! {vs_x16_swivel, stagger(10_000, 16), intersection_swivel}
    intersection_bench! {vs_x17,        stagger(10_000, 17)}
    intersection_bench! {vs_x17_future, stagger(10_000, 17), intersection_future}
    intersection_bench! {vs_x17_search, stagger(10_000, 17), intersection_search}
    intersection_bench! {vs_x17_stitch, stagger(10_000, 17), intersection_stitch}
    intersection_bench! {vs_x17_swivel, stagger(10_000, 17), intersection_swivel}
    intersection_bench! {vs_x18,        stagger(10_000, 18)}
    intersection_bench! {vs_x18_future, stagger(10_000, 18), intersection_future}
    intersection_bench! {vs_x18_search, stagger(10_000, 18), intersection_search}
    intersection_bench! {vs_x18_stitch, stagger(10_000, 18), intersection_stitch}
    intersection_bench! {vs_x18_swivel, stagger(10_000, 18), intersection_swivel}
    intersection_bench! {vs_x19,        stagger(10_000, 19)}
    intersection_bench! {vs_x19_future, stagger(10_000, 19), intersection_future}
    intersection_bench! {vs_x19_search, stagger(10_000, 19), intersection_search}
    intersection_bench! {vs_x19_stitch, stagger(10_000, 19), intersection_stitch}
    intersection_bench! {vs_x19_swivel, stagger(10_000, 19), intersection_swivel}
    intersection_bench! {vs_x20,        stagger(10_000, 20)}
    intersection_bench! {vs_x20_future, stagger(10_000, 20), intersection_future}
    intersection_bench! {vs_x20_search, stagger(10_000, 20), intersection_search}
    intersection_bench! {vs_x20_stitch, stagger(10_000, 20), intersection_stitch}
    intersection_bench! {vs_x20_swivel, stagger(10_000, 20), intersection_swivel}
}

#[cfg(feature = "include_100k")]
mod stagger_100_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(100_000, 15)}
    intersection_bench! {vs_x15_future, stagger(100_000, 15), intersection_future}
    intersection_bench! {vs_x15_search, stagger(100_000, 15), intersection_search}
    intersection_bench! {vs_x15_stitch, stagger(100_000, 15), intersection_stitch}
    intersection_bench! {vs_x15_swivel, stagger(100_000, 15), intersection_swivel}
    intersection_bench! {vs_x16,        stagger(100_000, 16)}
    intersection_bench! {vs_x16_future, stagger(100_000, 16), intersection_future}
    intersection_bench! {vs_x16_search, stagger(100_000, 16), intersection_search}
    intersection_bench! {vs_x16_stitch, stagger(100_000, 16), intersection_stitch}
    intersection_bench! {vs_x16_swivel, stagger(100_000, 16), intersection_swivel}
    intersection_bench! {vs_x17,        stagger(100_000, 17)}
    intersection_bench! {vs_x17_future, stagger(100_000, 17), intersection_future}
    intersection_bench! {vs_x17_search, stagger(100_000, 17), intersection_search}
    intersection_bench! {vs_x17_stitch, stagger(100_000, 17), intersection_stitch}
    intersection_bench! {vs_x17_swivel, stagger(100_000, 17), intersection_swivel}
    intersection_bench! {vs_x18,        stagger(100_000, 18)}
    intersection_bench! {vs_x18_future, stagger(100_000, 18), intersection_future}
    intersection_bench! {vs_x18_search, stagger(100_000, 18), intersection_search}
    intersection_bench! {vs_x18_stitch, stagger(100_000, 18), intersection_stitch}
    intersection_bench! {vs_x18_swivel, stagger(100_000, 18), intersection_swivel}
    intersection_bench! {vs_x19,        stagger(100_000, 19)}
    intersection_bench! {vs_x19_future, stagger(100_000, 19), intersection_future}
    intersection_bench! {vs_x19_search, stagger(100_000, 19), intersection_search}
    intersection_bench! {vs_x19_stitch, stagger(100_000, 19), intersection_stitch}
    intersection_bench! {vs_x19_swivel, stagger(100_000, 19), intersection_swivel}
    intersection_bench! {vs_x20,        stagger(100_000, 20)}
    intersection_bench! {vs_x20_future, stagger(100_000, 20), intersection_future}
    intersection_bench! {vs_x20_search, stagger(100_000, 20), intersection_search}
    intersection_bench! {vs_x20_stitch, stagger(100_000, 20), intersection_stitch}
    intersection_bench! {vs_x20_swivel, stagger(100_000, 20), intersection_swivel}
}
