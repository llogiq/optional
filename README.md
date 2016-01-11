# Optional

[![Build Status](https://travis-ci.org/llogiq/optional.svg)](https://travis-ci.org/llogiq/optional)
[![Current Version](http://meritbadge.herokuapp.com/optional)](https://crates.io/crates/optional)
[![License: MIT/Apache](https://img.shields.io/dub/l/vibe-d.svg)](LICENSE)

This package aims to offer some `Option<_>`-like values that allow a 
space-optimized representation.

As of now there is a type `OptionBool`. It works very much like an
`Option<bool>` (and indeed mostly copies its interface). There are only a few
additional methods where `Option<bool>` would recieve or return another 
`Option<bool>`, a `_bool` prefix is used to receive/return `OptionBool` 
instead.

Note that due to the internal conversion it is generally impossible to create
`iter_mut()` or `as_mut_slice()` methods. Therefore those methods cannot be
supported.

Not only does `OptionBool` improve space efficiency as opposed to 
`Option<bool>`, most methods also perform a little faster in my benchmarks. On 
nightly, On an Intel(R) Core(TM)2 Quad CPU Q8400 @ 
2.66GHz running LUbuntu 15.04 with a 3.19.0-23-generic kernel and Rust 
1.3.0-nightly as of 2015-07-13, I get the following results:

```
test bench_and_bool_opt        ... bench:       2,370 ns/iter (+/- 188)
test bench_and_opt             ... bench:       2,553 ns/iter (+/- 96)
test bench_and_std             ... bench:       2,423 ns/iter (+/- 2)
test bench_as_slice_iter_opt   ... bench:       2,701 ns/iter (+/- 4)
test bench_as_slice_opt        ... bench:       2,711 ns/iter (+/- 5)
test bench_as_slice_std        ... bench:       2,883 ns/iter (+/- 3)
test bench_from_opt            ... bench:       3,002 ns/iter (+/- 14)
test bench_is_some_opt         ... bench:       2,510 ns/iter (+/- 4)
test bench_is_some_std         ... bench:       2,252 ns/iter (+/- 3)
test bench_iter_opt            ... bench:       2,701 ns/iter (+/- 4)
test bench_iter_std            ... bench:       2,880 ns/iter (+/- 3)
test bench_map_invert_opt      ... bench:       4,211 ns/iter (+/- 6)
test bench_map_invert_opt_bool ... bench:       3,157 ns/iter (+/- 10)
test bench_map_invert_std      ... bench:       4,513 ns/iter (+/- 13)
test bench_map_or_else_opt     ... bench:       2,552 ns/iter (+/- 3)
test bench_map_or_else_std     ... bench:       3,452 ns/iter (+/- 16)
test bench_map_or_opt          ... bench:       2,776 ns/iter (+/- 4)
test bench_map_or_std          ... bench:       2,714 ns/iter (+/- 4)
test bench_or_bool_opt         ... bench:       2,152 ns/iter (+/- 146)
test bench_or_else_bool_opt    ... bench:       2,627 ns/iter (+/- 101)
test bench_or_else_opt         ... bench:       2,422 ns/iter (+/- 3)
test bench_or_else_std         ... bench:       2,893 ns/iter (+/- 29)
test bench_or_opt              ... bench:       3,200 ns/iter (+/- 4)
test bench_or_std              ... bench:       2,537 ns/iter (+/- 10)
test bench_to_opt              ... bench:       2,859 ns/iter (+/- 3)
test bench_unwrap_opt          ... bench:       2,706 ns/iter (+/- 3)
test bench_unwrap_or_else_opt  ... bench:       2,776 ns/iter (+/- 4)
test bench_unwrap_or_else_std  ... bench:       3,240 ns/iter (+/- 3)
test bench_unwrap_or_opt       ... bench:       2,555 ns/iter (+/- 2)
test bench_unwrap_or_std       ... bench:       3,206 ns/iter (+/- 31)
test bench_unwrap_std          ... bench:       3,153 ns/iter (+/- 3)
```

Apart from `OptionBool`, there is a struct `Optioned<T>` that is similar to 
Option but needs no additional storage, declaring a None value instead. Their 
are impls For `i8..64`, `u8..64`, `isize`, `usize`, `f32` and `f64`, and it's 
easy enough to Implement it for your own type (have a look at the Noned trait). 
The None value for the int types is their `MIN`, for the unsigned types is 
their `MAX` and for floats is `NAN` (regardless of sign).

# Documentation

[API documentation](http://llogiq.github.io/optional)

You also may want to have a look at the very well documented
[Option docs](http://doc.rust-lang.org/std/option/enum.Option.html), on which
this crate is based.

# License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
