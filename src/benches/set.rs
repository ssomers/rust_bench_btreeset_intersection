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
    intersection_bench! {_100_neg_vs_100_pos_switch, [neg(100), pos(100)], intersection_switch}
    intersection_bench! {_100_neg_vs_10k_pos,        [neg(100), pos(10_000)]}
    intersection_bench! {_100_neg_vs_10k_pos_switch, [neg(100), pos(10_000)], intersection_switch}
    intersection_bench! {_100_pos_vs_100_neg,        [pos(100), neg(100)]}
    intersection_bench! {_100_pos_vs_100_neg_switch, [pos(100), neg(100)], intersection_switch}
    intersection_bench! {_100_pos_vs_10k_neg,        [pos(100), neg(10_000)]}
    intersection_bench! {_100_pos_vs_10k_neg_switch, [pos(100), neg(10_000)], intersection_switch}
    intersection_bench! {_10k_neg_vs_100_pos,        [neg(10_000), pos(100)]}
    intersection_bench! {_10k_neg_vs_100_pos_switch, [neg(10_000), pos(100)], intersection_switch}
    intersection_bench! {_10k_neg_vs_10k_pos,        [neg(10_000), pos(10_000)]}
    intersection_bench! {_10k_neg_vs_10k_pos_switch, [neg(10_000), pos(10_000)], intersection_switch}
    intersection_bench! {_10k_pos_vs_100_neg,        [pos(10_000), neg(100)]}
    intersection_bench! {_10k_pos_vs_100_neg_switch, [pos(10_000), neg(100)], intersection_switch}
    intersection_bench! {_10k_pos_vs_10k_neg,        [pos(10_000), neg(10_000)]}
    intersection_bench! {_10k_pos_vs_10k_neg_switch, [pos(10_000), neg(10_000)], intersection_switch}
}

mod intersect_random_100 {
    use super::random;
    intersection_bench! {vs_100,            random(100, 100)}
    intersection_bench! {vs_100_switch,     random(100, 100), intersection_switch}
    intersection_bench! {vs_1600,           random(100, 1_600)}
    intersection_bench! {vs_1600_switch,    random(100, 1_600), intersection_switch}
    intersection_bench! {vs_10k,            random(100, 10_000)}
    intersection_bench! {vs_10k_switch,     random(100, 10_000), intersection_switch}
}

mod intersect_random_10k {
    use super::random;
    intersection_bench! {vs_10k,            random(10_000, 10_000)}
    intersection_bench! {vs_10k_switch,     random(10_000, 10_000), intersection_switch}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k,           random(10_000, 160_000)}
    #[cfg(feature = "include_100k")]
    intersection_bench! {vs_160k_switch,    random(10_000, 160_000), intersection_switch}
}

mod stagger_000_001 {
    use super::stagger;
    intersection_bench! {vs_1,          stagger(1, 1)}
    intersection_bench! {vs_1_switch,   stagger(1, 1), intersection_switch}
}

mod stagger_000_002 {
    use super::stagger;
    intersection_bench! {vs_2,          stagger(2, 1)}
    intersection_bench! {vs_2_switch,   stagger(2, 1), intersection_switch}
}

mod stagger_000_004 {
    use super::stagger;
    intersection_bench! {vs_4,          stagger(4, 1)}
    intersection_bench! {vs_4_switch,   stagger(4, 1), intersection_switch}
}

mod stagger_000_006 {
    use super::stagger;
    intersection_bench! {vs_6,          stagger(6, 1)}
    intersection_bench! {vs_6_switch,   stagger(6, 1), intersection_switch}
}

mod stagger_000_008 {
    use super::stagger;
    intersection_bench! {vs_8,          stagger(8, 1)}
    intersection_bench! {vs_8_switch,   stagger(8, 1), intersection_switch}
}

mod stagger_000_010 {
    use super::stagger;
    intersection_bench! {vs_x02,        stagger(10, 2)}
    intersection_bench! {vs_x02_switch, stagger(10, 2), intersection_switch}
    intersection_bench! {vs_x03,        stagger(10, 3)}
    intersection_bench! {vs_x03_switch, stagger(10, 3), intersection_switch}
    intersection_bench! {vs_x04,        stagger(10, 4)}
    intersection_bench! {vs_x04_switch, stagger(10, 4), intersection_switch}
    intersection_bench! {vs_x05,        stagger(10, 5)}
    intersection_bench! {vs_x05_switch, stagger(10, 5), intersection_switch}
    intersection_bench! {vs_x15,        stagger(10, 15)}
    intersection_bench! {vs_x15_switch, stagger(10, 15), intersection_switch}
    intersection_bench! {vs_x16,        stagger(10, 16)}
    intersection_bench! {vs_x16_switch, stagger(10, 16), intersection_switch}
}

mod stagger_000_100 {
    use super::stagger;
    intersection_bench! {vs_x04,        stagger(100, 4)}
    intersection_bench! {vs_x04_switch, stagger(100, 4), intersection_switch}
    intersection_bench! {vs_x05,        stagger(100, 5)}
    intersection_bench! {vs_x05_switch, stagger(100, 5), intersection_switch}
    intersection_bench! {vs_x06,        stagger(100, 6)}
    intersection_bench! {vs_x06_switch, stagger(100, 6), intersection_switch}
    intersection_bench! {vs_x07,        stagger(100, 7)}
    intersection_bench! {vs_x07_switch, stagger(100, 7), intersection_switch}
    intersection_bench! {vs_x15,        stagger(100, 15)}
    intersection_bench! {vs_x15_switch, stagger(100, 15), intersection_switch}
    intersection_bench! {vs_x16,        stagger(100, 16)}
    intersection_bench! {vs_x16_switch, stagger(100, 16), intersection_switch}
}

mod stagger_000_200 {
    use super::stagger;
    intersection_bench! {vs_x05,        stagger(200, 5)}
    intersection_bench! {vs_x05_switch, stagger(200, 5), intersection_switch}
    intersection_bench! {vs_x06,        stagger(200, 6)}
    intersection_bench! {vs_x06_switch, stagger(200, 6), intersection_switch}
    intersection_bench! {vs_x07,        stagger(200, 7)}
    intersection_bench! {vs_x07_switch, stagger(200, 7), intersection_switch}
    intersection_bench! {vs_x08,        stagger(200, 8)}
    intersection_bench! {vs_x08_switch, stagger(200, 8), intersection_switch}
    intersection_bench! {vs_x15,        stagger(200, 15)}
    intersection_bench! {vs_x15_switch, stagger(200, 15), intersection_switch}
    intersection_bench! {vs_x16,        stagger(200, 16)}
    intersection_bench! {vs_x16_switch, stagger(200, 16), intersection_switch}
}

mod stagger_000_500 {
    use super::stagger;
    intersection_bench! {vs_x12,        stagger(500, 12)}
    intersection_bench! {vs_x12_switch, stagger(500, 12), intersection_switch}
    intersection_bench! {vs_x13,        stagger(500, 13)}
    intersection_bench! {vs_x13_switch, stagger(500, 13), intersection_switch}
    intersection_bench! {vs_x14,        stagger(500, 14)}
    intersection_bench! {vs_x14_switch, stagger(500, 14), intersection_switch}
    intersection_bench! {vs_x15,        stagger(500, 15)}
    intersection_bench! {vs_x15_switch, stagger(500, 15), intersection_switch}
    intersection_bench! {vs_x16,        stagger(500, 16)}
    intersection_bench! {vs_x16_switch, stagger(500, 16), intersection_switch}
}

mod stagger_001_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(1_000, 15)}
    intersection_bench! {vs_x15_switch, stagger(1_000, 15), intersection_switch}
    intersection_bench! {vs_x16,        stagger(1_000, 16)}
    intersection_bench! {vs_x16_switch, stagger(1_000, 16), intersection_switch}
    intersection_bench! {vs_x17,        stagger(1_000, 17)}
    intersection_bench! {vs_x17_switch, stagger(1_000, 17), intersection_switch}
    intersection_bench! {vs_x18,        stagger(1_000, 18)}
    intersection_bench! {vs_x18_switch, stagger(1_000, 18), intersection_switch}
    intersection_bench! {vs_x19,        stagger(1_000, 19)}
    intersection_bench! {vs_x19_switch, stagger(1_000, 19), intersection_switch}
}

#[cfg(feature = "include_100k")]
mod stagger_010_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(10_000, 15)}
    intersection_bench! {vs_x15_switch, stagger(10_000, 15), intersection_switch}
    intersection_bench! {vs_x16,        stagger(10_000, 16)}
    intersection_bench! {vs_x16_switch, stagger(10_000, 16), intersection_switch}
    intersection_bench! {vs_x17,        stagger(10_000, 17)}
    intersection_bench! {vs_x17_switch, stagger(10_000, 17), intersection_switch}
    intersection_bench! {vs_x18,        stagger(10_000, 18)}
    intersection_bench! {vs_x18_switch, stagger(10_000, 18), intersection_switch}
    intersection_bench! {vs_x19,        stagger(10_000, 19)}
    intersection_bench! {vs_x19_switch, stagger(10_000, 19), intersection_switch}
    intersection_bench! {vs_x20,        stagger(10_000, 20)}
    intersection_bench! {vs_x20_switch, stagger(10_000, 20), intersection_switch}
}

#[cfg(feature = "include_100k")]
mod stagger_100_000 {
    use super::stagger;
    intersection_bench! {vs_x15,        stagger(100_000, 15)}
    intersection_bench! {vs_x15_switch, stagger(100_000, 15), intersection_switch}
    intersection_bench! {vs_x16,        stagger(100_000, 16)}
    intersection_bench! {vs_x16_switch, stagger(100_000, 16), intersection_switch}
    intersection_bench! {vs_x17,        stagger(100_000, 17)}
    intersection_bench! {vs_x17_switch, stagger(100_000, 17), intersection_switch}
    intersection_bench! {vs_x18,        stagger(100_000, 18)}
    intersection_bench! {vs_x18_switch, stagger(100_000, 18), intersection_switch}
    intersection_bench! {vs_x19,        stagger(100_000, 19)}
    intersection_bench! {vs_x19_switch, stagger(100_000, 19), intersection_switch}
    intersection_bench! {vs_x20,        stagger(100_000, 20)}
    intersection_bench! {vs_x20_switch, stagger(100_000, 20), intersection_switch}
}
