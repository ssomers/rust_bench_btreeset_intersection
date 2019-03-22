# rust_bench_btreeset_intersection
Case study comparing the performance of strategies for calculating the intersection of BTreeSet instances.
Requires a build that supports benchmarks like nightly.

`cargo bench` produces a bunch of measurement in groups of 5, for instance:

    test intersect_random_100_vs_100          ... bench:         702 ns/iter (+/- 35)
    test intersect_random_100_vs_100_future   ... bench:         473 ns/iter (+/- 29)
    test intersect_random_100_vs_100_search   ... bench:       1,534 ns/iter (+/- 64)
    test intersect_random_100_vs_100_spring   ... bench:       1,616 ns/iter (+/- 102)
    test intersect_random_100_vs_100_stitch   ... bench:         478 ns/iter (+/- 10)

Each of these 5 test measures the time spent intersecting two different sets with 100 pseudo-random elements (with the same seed each time), in order:
- on top: implementation of intersection in the liballoc of the (nightly) rustc build used
- future: proposed implementation, that attempts to choose wisely between one of the strategies below (assuming that the local build has the same optimizations as the nightly build)
- search: iterating over the smallest of the sets, each time searching for a match in the largest set
- spring: bock-spring implementation, searching for the element equal to or greater than the lower bound of the unvisited values in the other set (never used)
- stitch: same strategy as the original liballoc, but implemented more efficiently without Peekable

Tests named `intersect_stagger_500_vs_x16` intersect a set of 500 elements with a disjoint set of 8000 elements (500 times 16), with the elements spaced evenly (e.g. 0 in first set, 1..16 in second set, 17 in first set, etc). Comparing for various sizes allows estimating  a factor for which the search and the stitch strategy perform likewise:

[![Comparison](https://plot.ly/~stein.somers/216.png "View interactively")](https://plot.ly/~stein.somers/216)

The graph also shows how much we lose by choosing a constant factor 16, regardless of the size of the small set.
For instance, unless we'd pick a different factor or introduce a complicated rule involving absolute sizes:
- A 10 element set intersected with a 150 element set is almost 3 times slower than it could have been.
- A 10k element set intersected with a 1600k element set is almost 30% slower than it could have been.


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
