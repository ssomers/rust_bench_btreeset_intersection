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

fn subset(n1: usize, factor: usize) -> [BTreeSet<u32>; 2] {
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

fn superset(factor: usize, n2: usize) -> [BTreeSet<u32>; 2] {
    let n1 = n2 * factor;
    let mut sets = subset(n2, factor);
    sets.swap(0, 1);
    assert_eq!(sets[0].len(), n1);
    assert_eq!(sets[1].len(), n2);
    sets
}

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
    ($bench_name: ident, $sets: expr, $oper_name: path, $consume_name: ident) => {
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

macro_rules! set_benches {
    ($mod_name: ident, $oper_name: path, $consume_name: ident, $($bench_name: ident: $sets: expr;)+) => {
        mod $mod_name {
            $(
                set_bench!($bench_name, $sets, $oper_name, $consume_name);
            )+
        }
    }
}

macro_rules! vanilla_benches {
    ($mod_name: ident, $oper_name: path, $consume_name: ident) => {
        set_benches!($mod_name, $oper_name, $consume_name,
            equal_100_vs_100:           super::subset(100, 1);
            random_100_vs_100:          super::random(100, 100);
            random_100_vs_10k:          super::random(100, 10_000);
            random_100_vs_1600:         super::random(100, 1_600);
            random_10k_vs_10k:          super::random(10_000, 10_000);
            stagger_100_vs_100:         super::stagger(100, 1);
            stagger_100_vs_10k:         super::stagger(100, 100);
            split_100_neg_vs_100_pos:   [super::neg(100), super::pos(100)];
            split_100_neg_vs_10k_pos:   [super::neg(100), super::pos(10_000)];
            split_100_pos_vs_100_neg:   [super::pos(100), super::neg(100)];
            split_100_pos_vs_10k_neg:   [super::pos(100), super::neg(10_000)];
            split_10k_neg_vs_100_pos:   [super::neg(10_000), super::pos(100)];
            split_10k_neg_vs_10k_pos:   [super::neg(10_000), super::pos(10_000)];
            split_10k_pos_vs_100_neg:   [super::pos(10_000), super::neg(100)];
            split_10k_pos_vs_10k_neg:   [super::pos(10_000), super::neg(10_000)];
            subset_010_vs_100:          super::subset(10, 10);
            subset_100_vs_10k:          super::subset(100, 100);
            superset_100_vs_010:        super::superset(10, 10);
            superset_10k_vs_100:        super::superset(100, 100);
        );
    }
}

macro_rules! stagger_benches {
    ($mod_name: ident, $oper_name: path, $consume_name: ident) => {
        #[cfg(feature = "stagger")]
        set_benches!($mod_name, $oper_name, $consume_name,
            _000_001_vs_x01:    super::stagger(1, 1);
            _000_002_vs_x01:    super::stagger(2, 1);
            _000_004_vs_x01:    super::stagger(4, 1);
            _000_006_vs_x01:    super::stagger(6, 1);
            _000_008_vs_x01:    super::stagger(8, 1);
            _000_010_vs_x02:    super::stagger(10, 2);
            _000_010_vs_x03:    super::stagger(10, 3);
            _000_010_vs_x04:    super::stagger(10, 4);
            _000_010_vs_x05:    super::stagger(10, 5);
            _000_010_vs_x15:    super::stagger(10, 15);
            _000_010_vs_x16:    super::stagger(10, 16);
            _000_100_vs_x04:    super::stagger(100, 4);
            _000_100_vs_x05:    super::stagger(100, 5);
            _000_100_vs_x06:    super::stagger(100, 6);
            _000_100_vs_x07:    super::stagger(100, 7);
            _000_100_vs_x15:    super::stagger(100, 15);
            _000_100_vs_x16:    super::stagger(100, 16);
            _000_200_vs_x05:    super::stagger(200, 5);
            _000_200_vs_x06:    super::stagger(200, 6);
            _000_200_vs_x07:    super::stagger(200, 7);
            _000_200_vs_x08:    super::stagger(200, 8);
            _000_200_vs_x15:    super::stagger(200, 15);
            _000_200_vs_x16:    super::stagger(200, 16);
            _000_500_vs_x12:    super::stagger(500, 12);
            _000_500_vs_x13:    super::stagger(500, 13);
            _000_500_vs_x14:    super::stagger(500, 14);
            _000_500_vs_x15:    super::stagger(500, 15);
            _000_500_vs_x16:    super::stagger(500, 16);
            _001_000_vs_x15:    super::stagger(1_000, 15);
            _001_000_vs_x16:    super::stagger(1_000, 16);
            _001_000_vs_x17:    super::stagger(1_000, 17);
            _001_000_vs_x18:    super::stagger(1_000, 18);
            _001_000_vs_x19:    super::stagger(1_000, 19);
            _010_000_vs_x15:    super::stagger(10_000, 15);
            _010_000_vs_x16:    super::stagger(10_000, 16);
            _010_000_vs_x17:    super::stagger(10_000, 17);
            _010_000_vs_x18:    super::stagger(10_000, 18);
            _010_000_vs_x19:    super::stagger(10_000, 19);
            _010_000_vs_x20:    super::stagger(10_000, 20);
            _100_000_vs_x15:    super::stagger(100_000, 15);
            _100_000_vs_x16:    super::stagger(100_000, 16);
            _100_000_vs_x17:    super::stagger(100_000, 17);
            _100_000_vs_x18:    super::stagger(100_000, 18);
            _100_000_vs_x19:    super::stagger(100_000, 19);
            _100_000_vs_x20:    super::stagger(100_000, 20);
        );
    }
}

vanilla_benches! {dif_old, rust_bench_btreeset::set_now::difference, count}
vanilla_benches! {dif_new, rust_bench_btreeset::set_new::difference, count}
vanilla_benches! {dif_peeking, rust_bench_btreeset::set_peeking::difference, count}
vanilla_benches! {sub_old, rust_bench_btreeset::set_now::is_subset, clone}
vanilla_benches! {sub_new, rust_bench_btreeset::set_new::is_subset, clone}

vanilla_benches! {int_old, rust_bench_btreeset::set_now::intersection, count}
vanilla_benches! {int_new, rust_bench_btreeset::set_new::intersection, count}
vanilla_benches! {int_switch, rust_bench_btreeset::set_switch::intersection, count}
vanilla_benches! {int_swivel, rust_bench_btreeset::set_swivel::intersection, count}

vanilla_benches! {sym_old, rust_bench_btreeset::set_now::symmdiff, count}
vanilla_benches! {sym_new, rust_bench_btreeset::set_new::symmdiff, count}
vanilla_benches! {uni_old, rust_bench_btreeset::set_now::union, count}
vanilla_benches! {uni_new, rust_bench_btreeset::set_new::union, count}

stagger_benches! {int_stagger_old, rust_bench_btreeset::set_now::intersection, count}
stagger_benches! {int_stagger_new, rust_bench_btreeset::set_peeking::intersection, count}
stagger_benches! {int_stagger_search, rust_bench_btreeset::set_peeking::intersection_search, count}
stagger_benches! {int_stagger_stitch, rust_bench_btreeset::set_peeking::intersection_stitch, count}
