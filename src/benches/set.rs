// file comparable to rust/src/liballoc/benches/btree/set.rs
#![feature(test)]

use std::collections::BTreeSet;

extern crate rand;
extern crate test;
use self::rand::{thread_rng, Rng};
use self::test::{black_box, Bencher};

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

fn stagger(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
    let n2 = n1 * factor;
    let mut sets = [BTreeSet::new(), BTreeSet::new()];
    for i in 0..(n1 + n2) {
        let b = i % (factor + 1) != 0;
        sets[b as usize].insert(i as u32);
    }
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

macro_rules! intersection_bench {
    ($name: ident, $sets: expr) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            // setup
            let sets = $sets;

            // measure
            b.iter(|| {
                let x = sets[0].intersection(&sets[1]).count();
                black_box(x);
            })
        }
    };
    ($name: ident, $sets: expr, $intersection_kind: ident) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            use ::rust_bench_btreeset_intersection::set::$intersection_kind;

            // setup
            let sets = $sets;
            assert!(sets[0].len() <= sets[1].len());

            // measure
            b.iter(|| {
                let x = $intersection_kind(&sets[0], &sets[1]).count();
                black_box(x);
            })
        }
    };
}

intersection_bench! {intersect_random_100_vs_100,       random(100, 100)}
intersection_bench! {intersect_random_100_vs_100_future,random(100, 100), intersection_future}
intersection_bench! {intersect_random_100_vs_100_search,random(100, 100), intersection_search}
intersection_bench! {intersect_random_100_vs_100_spring,random(100, 100), intersection_spring}
intersection_bench! {intersect_random_100_vs_100_stitch,random(100, 100), intersection_stitch}
intersection_bench! {intersect_random_100_vs_1600,       random(100, 1_600)}
intersection_bench! {intersect_random_100_vs_1600_future,random(100, 1_600), intersection_future}
intersection_bench! {intersect_random_100_vs_1600_search,random(100, 1_600), intersection_search}
intersection_bench! {intersect_random_100_vs_1600_spring,random(100, 1_600), intersection_spring}
intersection_bench! {intersect_random_100_vs_1600_stitch,random(100, 1_600), intersection_stitch}
intersection_bench! {intersect_random_100_vs_10k,       random(100, 10_000)}
intersection_bench! {intersect_random_100_vs_10k_future,random(100, 10_000), intersection_future}
intersection_bench! {intersect_random_100_vs_10k_search,random(100, 10_000), intersection_search}
intersection_bench! {intersect_random_100_vs_10k_spring,random(100, 10_000), intersection_spring}
intersection_bench! {intersect_random_100_vs_10k_stitch,random(100, 10_000), intersection_stitch}
intersection_bench! {intersect_random_10k_vs_10k,       random(10_000, 10_000)}
intersection_bench! {intersect_random_10k_vs_10k_future,random(10_000, 10_000), intersection_future}
intersection_bench! {intersect_random_10k_vs_10k_search,random(10_000, 10_000), intersection_search}
intersection_bench! {intersect_random_10k_vs_10k_spring,random(10_000, 10_000), intersection_spring}
intersection_bench! {intersect_random_10k_vs_10k_stitch,random(10_000, 10_000), intersection_stitch}
intersection_bench! {intersect_random_10k_vs_160k,      random(10_000, 160_000)}
intersection_bench! {intersect_random_10k_vs_160k_future,random(10_000, 160_000), intersection_future}
intersection_bench! {intersect_random_10k_vs_160k_search,random(10_000, 160_000), intersection_search}
intersection_bench! {intersect_random_10k_vs_160k_spring,random(10_000, 160_000), intersection_spring}
intersection_bench! {intersect_random_10k_vs_160k_stitch,random(10_000, 160_000), intersection_stitch}
intersection_bench! {intersect_stagger_1_vs_1,          stagger(1, 1)}
intersection_bench! {intersect_stagger_1_vs_1_future,   stagger(1, 1), intersection_future}
intersection_bench! {intersect_stagger_1_vs_1_search,   stagger(1, 1), intersection_search}
intersection_bench! {intersect_stagger_1_vs_1_spring,   stagger(1, 1), intersection_spring}
intersection_bench! {intersect_stagger_1_vs_1_stitch,   stagger(1, 1), intersection_stitch}
intersection_bench! {intersect_stagger_10_vs_x2,        stagger(10, 2)}
intersection_bench! {intersect_stagger_10_vs_x2_future, stagger(10, 2), intersection_future}
intersection_bench! {intersect_stagger_10_vs_x2_search, stagger(10, 2), intersection_search}
intersection_bench! {intersect_stagger_10_vs_x2_spring, stagger(10, 2), intersection_spring}
intersection_bench! {intersect_stagger_10_vs_x2_stitch, stagger(10, 2), intersection_stitch}
intersection_bench! {intersect_stagger_10_vs_x3,        stagger(10, 3)}
intersection_bench! {intersect_stagger_10_vs_x3_future, stagger(10, 3), intersection_future}
intersection_bench! {intersect_stagger_10_vs_x3_search, stagger(10, 3), intersection_search}
intersection_bench! {intersect_stagger_10_vs_x3_spring, stagger(10, 3), intersection_spring}
intersection_bench! {intersect_stagger_10_vs_x3_stitch, stagger(10, 3), intersection_stitch}
intersection_bench! {intersect_stagger_10_vs_x4,        stagger(10, 4)}
intersection_bench! {intersect_stagger_10_vs_x4_future, stagger(10, 4), intersection_future}
intersection_bench! {intersect_stagger_10_vs_x4_search, stagger(10, 4), intersection_search}
intersection_bench! {intersect_stagger_10_vs_x4_spring, stagger(10, 4), intersection_spring}
intersection_bench! {intersect_stagger_10_vs_x4_stitch, stagger(10, 4), intersection_stitch}
intersection_bench! {intersect_stagger_10_vs_x5,        stagger(10, 5)}
intersection_bench! {intersect_stagger_10_vs_x5_future, stagger(10, 5), intersection_future}
intersection_bench! {intersect_stagger_10_vs_x5_search, stagger(10, 5), intersection_search}
intersection_bench! {intersect_stagger_10_vs_x5_spring, stagger(10, 5), intersection_spring}
intersection_bench! {intersect_stagger_10_vs_x5_stitch, stagger(10, 5), intersection_stitch}
intersection_bench! {intersect_stagger_10_vs_x15,       stagger(10, 15)}
intersection_bench! {intersect_stagger_10_vs_x15_future,stagger(10, 15), intersection_future}
intersection_bench! {intersect_stagger_10_vs_x15_search,stagger(10, 15), intersection_search}
intersection_bench! {intersect_stagger_10_vs_x15_spring,stagger(10, 15), intersection_spring}
intersection_bench! {intersect_stagger_10_vs_x15_stitch,stagger(10, 15), intersection_stitch}
intersection_bench! {intersect_stagger_10_vs_x16,       stagger(10, 16)}
intersection_bench! {intersect_stagger_10_vs_x16_future,stagger(10, 16), intersection_future}
intersection_bench! {intersect_stagger_10_vs_x16_search,stagger(10, 16), intersection_search}
intersection_bench! {intersect_stagger_10_vs_x16_spring,stagger(10, 16), intersection_spring}
intersection_bench! {intersect_stagger_10_vs_x16_stitch,stagger(10, 16), intersection_stitch}
intersection_bench! {intersect_stagger_100_vs_x4,       stagger(100, 4)}
intersection_bench! {intersect_stagger_100_vs_x4_future,stagger(100, 4), intersection_future}
intersection_bench! {intersect_stagger_100_vs_x4_search,stagger(100, 4), intersection_search}
intersection_bench! {intersect_stagger_100_vs_x4_spring,stagger(100, 4), intersection_spring}
intersection_bench! {intersect_stagger_100_vs_x4_stitch,stagger(100, 4), intersection_stitch}
intersection_bench! {intersect_stagger_100_vs_x5,       stagger(100, 5)}
intersection_bench! {intersect_stagger_100_vs_x5_future,stagger(100, 5), intersection_future}
intersection_bench! {intersect_stagger_100_vs_x5_search,stagger(100, 5), intersection_search}
intersection_bench! {intersect_stagger_100_vs_x5_spring,stagger(100, 5), intersection_spring}
intersection_bench! {intersect_stagger_100_vs_x5_stitch,stagger(100, 5), intersection_stitch}
intersection_bench! {intersect_stagger_100_vs_x6,       stagger(100, 6)}
intersection_bench! {intersect_stagger_100_vs_x6_future,stagger(100, 6), intersection_future}
intersection_bench! {intersect_stagger_100_vs_x6_search,stagger(100, 6), intersection_search}
intersection_bench! {intersect_stagger_100_vs_x6_spring,stagger(100, 6), intersection_spring}
intersection_bench! {intersect_stagger_100_vs_x6_stitch,stagger(100, 6), intersection_stitch}
intersection_bench! {intersect_stagger_100_vs_x7,       stagger(100, 7)}
intersection_bench! {intersect_stagger_100_vs_x7_future,stagger(100, 7), intersection_future}
intersection_bench! {intersect_stagger_100_vs_x7_search,stagger(100, 7), intersection_search}
intersection_bench! {intersect_stagger_100_vs_x7_spring,stagger(100, 7), intersection_spring}
intersection_bench! {intersect_stagger_100_vs_x7_stitch,stagger(100, 7), intersection_stitch}
intersection_bench! {intersect_stagger_100_vs_x15,       stagger(100, 15)}
intersection_bench! {intersect_stagger_100_vs_x15_future,stagger(100, 15), intersection_future}
intersection_bench! {intersect_stagger_100_vs_x15_search,stagger(100, 15), intersection_search}
intersection_bench! {intersect_stagger_100_vs_x15_spring,stagger(100, 15), intersection_spring}
intersection_bench! {intersect_stagger_100_vs_x15_stitch,stagger(100, 15), intersection_stitch}
intersection_bench! {intersect_stagger_100_vs_x16,       stagger(100, 16)}
intersection_bench! {intersect_stagger_100_vs_x16_future,stagger(100, 16), intersection_future}
intersection_bench! {intersect_stagger_100_vs_x16_search,stagger(100, 16), intersection_search}
intersection_bench! {intersect_stagger_100_vs_x16_spring,stagger(100, 16), intersection_spring}
intersection_bench! {intersect_stagger_100_vs_x16_stitch,stagger(100, 16), intersection_stitch}
intersection_bench! {intersect_stagger_200_vs_x5,       stagger(200, 5)}
intersection_bench! {intersect_stagger_200_vs_x5_future,stagger(200, 5), intersection_future}
intersection_bench! {intersect_stagger_200_vs_x5_search,stagger(200, 5), intersection_search}
intersection_bench! {intersect_stagger_200_vs_x5_spring,stagger(200, 5), intersection_spring}
intersection_bench! {intersect_stagger_200_vs_x5_stitch,stagger(200, 5), intersection_stitch}
intersection_bench! {intersect_stagger_200_vs_x6,       stagger(200, 6)}
intersection_bench! {intersect_stagger_200_vs_x6_future,stagger(200, 6), intersection_future}
intersection_bench! {intersect_stagger_200_vs_x6_search,stagger(200, 6), intersection_search}
intersection_bench! {intersect_stagger_200_vs_x6_spring,stagger(200, 6), intersection_spring}
intersection_bench! {intersect_stagger_200_vs_x6_stitch,stagger(200, 6), intersection_stitch}
intersection_bench! {intersect_stagger_200_vs_x7,       stagger(200, 7)}
intersection_bench! {intersect_stagger_200_vs_x7_future,stagger(200, 7), intersection_future}
intersection_bench! {intersect_stagger_200_vs_x7_search,stagger(200, 7), intersection_search}
intersection_bench! {intersect_stagger_200_vs_x7_spring,stagger(200, 7), intersection_spring}
intersection_bench! {intersect_stagger_200_vs_x7_stitch,stagger(200, 7), intersection_stitch}
intersection_bench! {intersect_stagger_200_vs_x8,       stagger(200, 8)}
intersection_bench! {intersect_stagger_200_vs_x8_future,stagger(200, 8), intersection_future}
intersection_bench! {intersect_stagger_200_vs_x8_search,stagger(200, 8), intersection_search}
intersection_bench! {intersect_stagger_200_vs_x8_spring,stagger(200, 8), intersection_spring}
intersection_bench! {intersect_stagger_200_vs_x8_stitch,stagger(200, 8), intersection_stitch}
intersection_bench! {intersect_stagger_200_vs_x15,       stagger(200, 15)}
intersection_bench! {intersect_stagger_200_vs_x15_future,stagger(200, 15), intersection_future}
intersection_bench! {intersect_stagger_200_vs_x15_search,stagger(200, 15), intersection_search}
intersection_bench! {intersect_stagger_200_vs_x15_spring,stagger(200, 15), intersection_spring}
intersection_bench! {intersect_stagger_200_vs_x15_stitch,stagger(200, 15), intersection_stitch}
intersection_bench! {intersect_stagger_200_vs_x16,       stagger(200, 16)}
intersection_bench! {intersect_stagger_200_vs_x16_future,stagger(200, 16), intersection_future}
intersection_bench! {intersect_stagger_200_vs_x16_search,stagger(200, 16), intersection_search}
intersection_bench! {intersect_stagger_200_vs_x16_spring,stagger(200, 16), intersection_spring}
intersection_bench! {intersect_stagger_200_vs_x16_stitch,stagger(200, 16), intersection_stitch}
intersection_bench! {intersect_stagger_500_vs_x12,       stagger(500, 12)}
intersection_bench! {intersect_stagger_500_vs_x12_future,stagger(500, 12), intersection_future}
intersection_bench! {intersect_stagger_500_vs_x12_search,stagger(500, 12), intersection_search}
intersection_bench! {intersect_stagger_500_vs_x12_spring,stagger(500, 12), intersection_spring}
intersection_bench! {intersect_stagger_500_vs_x12_stitch,stagger(500, 12), intersection_stitch}
intersection_bench! {intersect_stagger_500_vs_x13,       stagger(500, 13)}
intersection_bench! {intersect_stagger_500_vs_x13_future,stagger(500, 13), intersection_future}
intersection_bench! {intersect_stagger_500_vs_x13_search,stagger(500, 13), intersection_search}
intersection_bench! {intersect_stagger_500_vs_x13_spring,stagger(500, 13), intersection_spring}
intersection_bench! {intersect_stagger_500_vs_x13_stitch,stagger(500, 13), intersection_stitch}
intersection_bench! {intersect_stagger_500_vs_x14,       stagger(500, 14)}
intersection_bench! {intersect_stagger_500_vs_x14_future,stagger(500, 14), intersection_future}
intersection_bench! {intersect_stagger_500_vs_x14_search,stagger(500, 14), intersection_search}
intersection_bench! {intersect_stagger_500_vs_x14_spring,stagger(500, 14), intersection_spring}
intersection_bench! {intersect_stagger_500_vs_x14_stitch,stagger(500, 14), intersection_stitch}
intersection_bench! {intersect_stagger_500_vs_x15,       stagger(500, 15)}
intersection_bench! {intersect_stagger_500_vs_x15_future,stagger(500, 15), intersection_future}
intersection_bench! {intersect_stagger_500_vs_x15_search,stagger(500, 15), intersection_search}
intersection_bench! {intersect_stagger_500_vs_x15_spring,stagger(500, 15), intersection_spring}
intersection_bench! {intersect_stagger_500_vs_x15_stitch,stagger(500, 15), intersection_stitch}
intersection_bench! {intersect_stagger_500_vs_x16,       stagger(500, 16)}
intersection_bench! {intersect_stagger_500_vs_x16_future,stagger(500, 16), intersection_future}
intersection_bench! {intersect_stagger_500_vs_x16_search,stagger(500, 16), intersection_search}
intersection_bench! {intersect_stagger_500_vs_x16_spring,stagger(500, 16), intersection_spring}
intersection_bench! {intersect_stagger_500_vs_x16_stitch,stagger(500, 16), intersection_stitch}
intersection_bench! {intersect_stagger_1k_vs_x15,       stagger(1_000, 15)}
intersection_bench! {intersect_stagger_1k_vs_x15_future,stagger(1_000, 15), intersection_future}
intersection_bench! {intersect_stagger_1k_vs_x15_search,stagger(1_000, 15), intersection_search}
intersection_bench! {intersect_stagger_1k_vs_x15_spring,stagger(1_000, 15), intersection_spring}
intersection_bench! {intersect_stagger_1k_vs_x15_stitch,stagger(1_000, 15), intersection_stitch}
intersection_bench! {intersect_stagger_1k_vs_x16,       stagger(1_000, 16)}
intersection_bench! {intersect_stagger_1k_vs_x16_future,stagger(1_000, 16), intersection_future}
intersection_bench! {intersect_stagger_1k_vs_x16_search,stagger(1_000, 16), intersection_search}
intersection_bench! {intersect_stagger_1k_vs_x16_spring,stagger(1_000, 16), intersection_spring}
intersection_bench! {intersect_stagger_1k_vs_x16_stitch,stagger(1_000, 16), intersection_stitch}
intersection_bench! {intersect_stagger_1k_vs_x17,       stagger(1_000, 17)}
intersection_bench! {intersect_stagger_1k_vs_x17_future,stagger(1_000, 17), intersection_future}
intersection_bench! {intersect_stagger_1k_vs_x17_search,stagger(1_000, 17), intersection_search}
intersection_bench! {intersect_stagger_1k_vs_x17_spring,stagger(1_000, 17), intersection_spring}
intersection_bench! {intersect_stagger_1k_vs_x17_stitch,stagger(1_000, 17), intersection_stitch}
intersection_bench! {intersect_stagger_1k_vs_x18,       stagger(1_000, 18)}
intersection_bench! {intersect_stagger_1k_vs_x18_future,stagger(1_000, 18), intersection_future}
intersection_bench! {intersect_stagger_1k_vs_x18_search,stagger(1_000, 18), intersection_search}
intersection_bench! {intersect_stagger_1k_vs_x18_spring,stagger(1_000, 18), intersection_spring}
intersection_bench! {intersect_stagger_1k_vs_x18_stitch,stagger(1_000, 18), intersection_stitch}
intersection_bench! {intersect_stagger_1k_vs_x19,       stagger(1_000, 19)}
intersection_bench! {intersect_stagger_1k_vs_x19_future,stagger(1_000, 19), intersection_future}
intersection_bench! {intersect_stagger_1k_vs_x19_search,stagger(1_000, 19), intersection_search}
intersection_bench! {intersect_stagger_1k_vs_x19_spring,stagger(1_000, 19), intersection_spring}
intersection_bench! {intersect_stagger_1k_vs_x19_stitch,stagger(1_000, 19), intersection_stitch}
intersection_bench! {intersect_stagger_10k_vs_x15,       stagger(10_000, 15)}
intersection_bench! {intersect_stagger_10k_vs_x15_future,stagger(10_000, 15), intersection_future}
intersection_bench! {intersect_stagger_10k_vs_x15_search,stagger(10_000, 15), intersection_search}
intersection_bench! {intersect_stagger_10k_vs_x15_spring,stagger(10_000, 15), intersection_spring}
intersection_bench! {intersect_stagger_10k_vs_x15_stitch,stagger(10_000, 15), intersection_stitch}
intersection_bench! {intersect_stagger_10k_vs_x16,       stagger(10_000, 16)}
intersection_bench! {intersect_stagger_10k_vs_x16_future,stagger(10_000, 16), intersection_future}
intersection_bench! {intersect_stagger_10k_vs_x16_search,stagger(10_000, 16), intersection_search}
intersection_bench! {intersect_stagger_10k_vs_x16_spring,stagger(10_000, 16), intersection_spring}
intersection_bench! {intersect_stagger_10k_vs_x16_stitch,stagger(10_000, 16), intersection_stitch}
intersection_bench! {intersect_stagger_10k_vs_x17,       stagger(10_000, 17)}
intersection_bench! {intersect_stagger_10k_vs_x17_future,stagger(10_000, 17), intersection_future}
intersection_bench! {intersect_stagger_10k_vs_x17_search,stagger(10_000, 17), intersection_search}
intersection_bench! {intersect_stagger_10k_vs_x17_spring,stagger(10_000, 17), intersection_spring}
intersection_bench! {intersect_stagger_10k_vs_x17_stitch,stagger(10_000, 17), intersection_stitch}
intersection_bench! {intersect_stagger_10k_vs_x18,       stagger(10_000, 18)}
intersection_bench! {intersect_stagger_10k_vs_x18_future,stagger(10_000, 18), intersection_future}
intersection_bench! {intersect_stagger_10k_vs_x18_search,stagger(10_000, 18), intersection_search}
intersection_bench! {intersect_stagger_10k_vs_x18_spring,stagger(10_000, 18), intersection_spring}
intersection_bench! {intersect_stagger_10k_vs_x18_stitch,stagger(10_000, 18), intersection_stitch}
intersection_bench! {intersect_stagger_10k_vs_x19,       stagger(10_000, 19)}
intersection_bench! {intersect_stagger_10k_vs_x19_future,stagger(10_000, 19), intersection_future}
intersection_bench! {intersect_stagger_10k_vs_x19_search,stagger(10_000, 19), intersection_search}
intersection_bench! {intersect_stagger_10k_vs_x19_spring,stagger(10_000, 19), intersection_spring}
intersection_bench! {intersect_stagger_10k_vs_x19_stitch,stagger(10_000, 19), intersection_stitch}
intersection_bench! {intersect_stagger_10k_vs_x20,       stagger(10_000, 20)}
intersection_bench! {intersect_stagger_10k_vs_x20_future,stagger(10_000, 20), intersection_future}
intersection_bench! {intersect_stagger_10k_vs_x20_search,stagger(10_000, 20), intersection_search}
intersection_bench! {intersect_stagger_10k_vs_x20_spring,stagger(10_000, 20), intersection_spring}
intersection_bench! {intersect_stagger_10k_vs_x20_stitch,stagger(10_000, 20), intersection_stitch}
intersection_bench! {intersect_stagger_100k_vs_x15,       stagger(100_000, 15)}
intersection_bench! {intersect_stagger_100k_vs_x15_future,stagger(100_000, 15), intersection_future}
intersection_bench! {intersect_stagger_100k_vs_x15_search,stagger(100_000, 15), intersection_search}
intersection_bench! {intersect_stagger_100k_vs_x15_spring,stagger(100_000, 15), intersection_spring}
intersection_bench! {intersect_stagger_100k_vs_x15_stitch,stagger(100_000, 15), intersection_stitch}
intersection_bench! {intersect_stagger_100k_vs_x16,       stagger(100_000, 16)}
intersection_bench! {intersect_stagger_100k_vs_x16_future,stagger(100_000, 16), intersection_future}
intersection_bench! {intersect_stagger_100k_vs_x16_search,stagger(100_000, 16), intersection_search}
intersection_bench! {intersect_stagger_100k_vs_x16_spring,stagger(100_000, 16), intersection_spring}
intersection_bench! {intersect_stagger_100k_vs_x16_stitch,stagger(100_000, 16), intersection_stitch}
intersection_bench! {intersect_stagger_100k_vs_x17,       stagger(100_000, 17)}
intersection_bench! {intersect_stagger_100k_vs_x17_future,stagger(100_000, 17), intersection_future}
intersection_bench! {intersect_stagger_100k_vs_x17_search,stagger(100_000, 17), intersection_search}
intersection_bench! {intersect_stagger_100k_vs_x17_spring,stagger(100_000, 17), intersection_spring}
intersection_bench! {intersect_stagger_100k_vs_x17_stitch,stagger(100_000, 17), intersection_stitch}
intersection_bench! {intersect_stagger_100k_vs_x18,       stagger(100_000, 18)}
intersection_bench! {intersect_stagger_100k_vs_x18_future,stagger(100_000, 18), intersection_future}
intersection_bench! {intersect_stagger_100k_vs_x18_search,stagger(100_000, 18), intersection_search}
intersection_bench! {intersect_stagger_100k_vs_x18_spring,stagger(100_000, 18), intersection_spring}
intersection_bench! {intersect_stagger_100k_vs_x18_stitch,stagger(100_000, 18), intersection_stitch}
intersection_bench! {intersect_stagger_100k_vs_x19,       stagger(100_000, 19)}
intersection_bench! {intersect_stagger_100k_vs_x19_future,stagger(100_000, 19), intersection_future}
intersection_bench! {intersect_stagger_100k_vs_x19_search,stagger(100_000, 19), intersection_search}
intersection_bench! {intersect_stagger_100k_vs_x19_spring,stagger(100_000, 19), intersection_spring}
intersection_bench! {intersect_stagger_100k_vs_x19_stitch,stagger(100_000, 19), intersection_stitch}
intersection_bench! {intersect_stagger_100k_vs_x20,       stagger(100_000, 20)}
intersection_bench! {intersect_stagger_100k_vs_x20_future,stagger(100_000, 20), intersection_future}
intersection_bench! {intersect_stagger_100k_vs_x20_search,stagger(100_000, 20), intersection_search}
intersection_bench! {intersect_stagger_100k_vs_x20_spring,stagger(100_000, 20), intersection_spring}
intersection_bench! {intersect_stagger_100k_vs_x20_stitch,stagger(100_000, 20), intersection_stitch}
