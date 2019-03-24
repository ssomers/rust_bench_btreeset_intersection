# Rust BTreeSet intersection benchmark
Case study comparing the performance of strategies for calculating the intersection of BTreeSet instances.
Requires a build that supports benchmarks like nightly.

`cargo bench --features include_100k` produces a bunch of measurement in groups of 5, for instance:

    test intersect_random_100::vs_100         ... bench:         674 ns/iter (+/- 12)
    test intersect_random_100::vs_100_future  ... bench:         476 ns/iter (+/- 3)
    test intersect_random_100::vs_100_search  ... bench:       1,589 ns/iter (+/- 67)
    test intersect_random_100::vs_100_spring  ... bench:       1,535 ns/iter (+/- 73)
    test intersect_random_100::vs_100_stitch  ... bench:         468 ns/iter (+/- 11)

Each of these 5 test measures the time spent intersecting two different sets with 100 pseudo-random elements (with the same seed each time), in order:
- on top: implementation of intersection in the liballoc of the (nightly) rustc build used
- future: proposed implementation, that attempts to choose wisely between one of the strategies below (assuming that the local build has the same optimizations as the nightly build)
- search: iterating over the smallest of the sets, each time searching for a match in the largest set
- spring: bock-spring implementation, searching for the element equal to or greater than the lower bound of the unvisited values in the other set (never used)
- stitch: same strategy as the original liballoc, but implemented more efficiently without Peekable

Tests named `intersect_stagger_500_vs_x16` intersect a set of 500 elements with a disjoint set of 8000 elements (500 times 16), with the elements spaced evenly (e.g. 0 in first set, 1..16 in second set, 17 in first set, etc). Comparing for various sizes allows estimating  a factor for which the search and the stitch strategy perform likewise:

[![Comparison](https://plot.ly/~stein.somers/216.png "View interactively")](https://plot.ly/~stein.somers/216)

The graph also shows how much we lose by choosing a constant factor 16, regardless of the size of the small set.
For instance:
- A 10 element set intersected with a 160 element set (implying the search strategy) is almost 3 times faster than it was originally.
- A 10 element set intersected with a 150 element set (implying the stitch strategy) is almost 3 times slower than it could have been with a lower factor.
- A 10k element set intersected with a 1600k element set (implying the search strategy) is almost 30% slower than it could have been with a higher factor. It's also slower than it was originally, but only by 15%, because the stitch strategy compared to is some 15% faster than the original.


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
