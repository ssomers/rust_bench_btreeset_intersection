# Rust BTreeSet benchmark [![Build Status](https://travis-ci.com/ssomers/rust_bench_btreeset_intersection.svg?branch=master)](https://travis-ci.com/ssomers/rust_bench_btreeset_intersection?branch=master)

Case study comparing the performance of various ways strategies to calculate the intersection, and other binary operations, on BTreeSet instances.
Requires a build that supports benchmarks, like nightly.

`cargo bench --features diff,intersect,merge,stagger` produces a bunch of measurements, for instance:

    test intersect_new::random_100_vs_100            ... bench:         547 ns/iter (+/- 14)
    test intersect_old::random_100_vs_100            ... bench:         555 ns/iter (+/- 15)

Each of these benches measures the time spent intersecting two different sets with 100 pseudo-random elements (with the same seed each time), in order:
- _old: (local copy of) existing implementation in liballoc 
- _new: some new rule
- _search: iterating over the smallest of the sets, each time searching for a match in the largest set
- _stitch: same strategy as the original liballoc, but implemented more efficiently without Peekable
- _switch: stitch that switches to search when it (hopefully) becomes faster
- _swivel: bock-spring implementation, each time searching for the element equal to or greater than the lower bound of the unvisited values in the other set (never used)

It's best to direct the output to file, and run [cargo-benchcmp](https://github.com/BurntSushi/cargo-benchcmp) on it, .e.g:
    
    cargo bench --features diff,intersect,merge >bench.txt
    cargo benchcmp intersect_old:: intersect_new:: bench.txt --threshold 5
    
## Stagger

Tests named `intersect_stagger_new::_000_500_vs_x16` intersect a set of 500 elements with a disjoint set of 8000 elements (500 times 16), with the elements spaced evenly (e.g. 0 in first set, 1..16 in second set, 17 in first set, etc). Comparing for various sizes allows estimating a factor for which the search and the stitch strategy perform likewise:

[![Comparison](https://plot.ly/~stein.somers/216.png "View interactively")](https://plot.ly/~stein.somers/216)

The graph also shows how much we lose by choosing a constant factor 16, regardless of the size of the small set.
For instance:
- A 10 element set intersected with a 160 element set (implying the search strategy) is almost 3 times faster than it was originally.
- A 10 element set intersected with a 150 element set (implying the stitch strategy) is almost 3 times slower than it could have been with a lower factor.
- A 10k element set intersected with a 1600k element set (implying the search strategy) is almost 30% slower than it could have been with a higher factor. It's also slower than it was originally, but only by 15%, because the stitch strategy compared to is some 15% faster than the original stitch. And beware it's a microbenchmark: it preys on caches filled with its data and doesn't care how much other data gets pushed out. The search strategy should access less memory than the stitch strategy.


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
