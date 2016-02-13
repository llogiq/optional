# Optional

[![Build Status](https://travis-ci.org/llogiq/optional.svg)](https://travis-ci.org/llogiq/optional)
[![Coverage Status](https://coveralls.io/repos/llogiq/optional/badge.svg?branch=master&service=github)](https://coveralls.io/github/llogiq/optional?branch=master)
[![Current Version](http://meritbadge.herokuapp.com/optional)](https://crates.io/crates/optional)
[![License: MIT/Apache](https://img.shields.io/crates/l/optional.svg)](#License)

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
1.8.0-nightly as of 2016-01-22, I get the following results:

```
test bench_and_bool_opt        ... bench:       1,490 ns/iter (+/- 38)
test bench_and_opt             ... bench:       1,667 ns/iter (+/- 37)
test bench_and_std             ... bench:       1,695 ns/iter (+/- 30)
test bench_as_slice_iter_opt   ... bench:       2,211 ns/iter (+/- 16)
test bench_as_slice_opt        ... bench:       1,507 ns/iter (+/- 23)
test bench_from_opt            ... bench:       1,795 ns/iter (+/- 137)
test bench_is_some_opt         ... bench:       1,443 ns/iter (+/- 17)
test bench_is_some_std         ... bench:       1,659 ns/iter (+/- 17)
test bench_iter_opt            ... bench:       2,212 ns/iter (+/- 39)
test bench_iter_std            ... bench:       1,553 ns/iter (+/- 22)
test bench_map_invert_opt      ... bench:       2,918 ns/iter (+/- 35)
test bench_map_invert_opt_bool ... bench:       2,063 ns/iter (+/- 23)
test bench_map_invert_std      ... bench:       2,510 ns/iter (+/- 36)
test bench_map_or_else_opt     ... bench:       2,268 ns/iter (+/- 35)
test bench_map_or_else_std     ... bench:       2,309 ns/iter (+/- 33)
test bench_map_or_opt          ... bench:       1,669 ns/iter (+/- 19)
test bench_map_or_std          ... bench:       1,975 ns/iter (+/- 26)
test bench_or_bool_opt         ... bench:       1,424 ns/iter (+/- 33)
test bench_or_else_bool_opt    ... bench:       1,654 ns/iter (+/- 30)
test bench_or_else_opt         ... bench:       1,766 ns/iter (+/- 24)
test bench_or_else_std         ... bench:       1,529 ns/iter (+/- 13)
test bench_or_opt              ... bench:       1,806 ns/iter (+/- 50)
test bench_or_std              ... bench:       1,729 ns/iter (+/- 28)
test bench_to_opt              ... bench:       1,979 ns/iter (+/- 87)
test bench_unwrap_opt          ... bench:       1,700 ns/iter (+/- 29)
test bench_unwrap_or_else_opt  ... bench:       1,666 ns/iter (+/- 19)
test bench_unwrap_or_else_std  ... bench:       2,149 ns/iter (+/- 31)
test bench_unwrap_or_opt       ... bench:       1,653 ns/iter (+/- 23)
test bench_unwrap_or_std       ... bench:       2,124 ns/iter (+/- 30)
test bench_unwrap_std          ... bench:       1,645 ns/iter (+/- 20)

[...]

test bench_as_slice_iter_opt_f32 ... bench:       2,090 ns/iter (+/- 35)
test bench_as_slice_iter_opt_f64 ... bench:       1,870 ns/iter (+/- 33)
test bench_as_slice_iter_opt_i16 ... bench:       1,937 ns/iter (+/- 24)
test bench_as_slice_iter_opt_i32 ... bench:       2,028 ns/iter (+/- 47)
test bench_as_slice_iter_opt_i64 ... bench:       1,680 ns/iter (+/- 32)
test bench_as_slice_iter_opt_i8  ... bench:       2,032 ns/iter (+/- 16)
test bench_as_slice_iter_opt_u16 ... bench:       1,937 ns/iter (+/- 28)
test bench_as_slice_iter_opt_u32 ... bench:       2,030 ns/iter (+/- 59)
test bench_as_slice_iter_opt_u64 ... bench:       1,775 ns/iter (+/- 135)
test bench_as_slice_iter_opt_u8  ... bench:       2,031 ns/iter (+/- 23)
test bench_is_some_opt_f32       ... bench:       1,721 ns/iter (+/- 17)
test bench_is_some_opt_f64       ... bench:       1,721 ns/iter (+/- 24)
test bench_is_some_opt_u16       ... bench:       1,711 ns/iter (+/- 26)
test bench_is_some_opt_u32       ... bench:       1,677 ns/iter (+/- 25)
test bench_is_some_opt_u64       ... bench:       1,664 ns/iter (+/- 31)
test bench_is_some_opt_u8        ... bench:       1,724 ns/iter (+/- 22)
test bench_is_some_std_f32       ... bench:       1,466 ns/iter (+/- 22)
test bench_is_some_std_f64       ... bench:       1,465 ns/iter (+/- 18)
test bench_is_some_std_u16       ... bench:       1,470 ns/iter (+/- 29)
test bench_is_some_std_u32       ... bench:       1,465 ns/iter (+/- 18)
test bench_is_some_std_u64       ... bench:       1,474 ns/iter (+/- 61)
test bench_is_some_std_u8        ... bench:       1,850 ns/iter (+/- 35)
test bench_iter_opt_f32          ... bench:       1,828 ns/iter (+/- 17)
test bench_iter_opt_f64          ... bench:       1,845 ns/iter (+/- 25)
test bench_iter_opt_i16          ... bench:       1,755 ns/iter (+/- 17)
test bench_iter_opt_i32          ... bench:       1,755 ns/iter (+/- 18)
test bench_iter_opt_i64          ... bench:       1,547 ns/iter (+/- 43)
test bench_iter_opt_i8           ... bench:       1,754 ns/iter (+/- 17)
test bench_iter_opt_u16          ... bench:       1,754 ns/iter (+/- 21)
test bench_iter_opt_u32          ... bench:       1,563 ns/iter (+/- 16)
test bench_iter_opt_u64          ... bench:       1,534 ns/iter (+/- 20)
test bench_iter_opt_u8           ... bench:       1,754 ns/iter (+/- 18)
test bench_iter_std_f32          ... bench:       1,563 ns/iter (+/- 39)
test bench_iter_std_f64          ... bench:       1,587 ns/iter (+/- 17)
test bench_iter_std_i16          ... bench:       1,487 ns/iter (+/- 32)
test bench_iter_std_i32          ... bench:       1,587 ns/iter (+/- 20)
test bench_iter_std_i64          ... bench:       1,587 ns/iter (+/- 25)
test bench_iter_std_i8           ... bench:       1,631 ns/iter (+/- 98)
test bench_iter_std_u16          ... bench:       1,488 ns/iter (+/- 23)
test bench_iter_std_u32          ... bench:       1,590 ns/iter (+/- 59)
test bench_iter_std_u64          ... bench:       1,587 ns/iter (+/- 25)
test bench_iter_std_u8           ... bench:       1,623 ns/iter (+/- 72)
test bench_map_opt_f32           ... bench:       1,959 ns/iter (+/- 67)
test bench_map_opt_f64           ... bench:       1,654 ns/iter (+/- 40)
test bench_map_opt_u16           ... bench:       1,789 ns/iter (+/- 31)
test bench_map_opt_u32           ... bench:       2,431 ns/iter (+/- 43)
test bench_map_opt_u64           ... bench:       1,432 ns/iter (+/- 32)
test bench_map_opt_u8            ... bench:       1,813 ns/iter (+/- 18)
test bench_map_or_opt_f32        ... bench:       1,623 ns/iter (+/- 17)
test bench_map_or_opt_f64        ... bench:       1,669 ns/iter (+/- 22)
test bench_map_or_opt_u16        ... bench:       1,717 ns/iter (+/- 18)
test bench_map_or_opt_u32        ... bench:       1,640 ns/iter (+/- 19)
test bench_map_or_opt_u64        ... bench:       1,579 ns/iter (+/- 40)
test bench_map_or_opt_u8         ... bench:       1,814 ns/iter (+/- 20)
test bench_map_or_std_f32        ... bench:       1,537 ns/iter (+/- 28)
test bench_map_or_std_f64        ... bench:       1,777 ns/iter (+/- 28)
test bench_map_or_std_u16        ... bench:       1,931 ns/iter (+/- 37)
test bench_map_or_std_u32        ... bench:       1,807 ns/iter (+/- 22)
test bench_map_or_std_u64        ... bench:       1,786 ns/iter (+/- 15)
test bench_map_or_std_u8         ... bench:       2,010 ns/iter (+/- 19)
test bench_map_std_f32           ... bench:       1,811 ns/iter (+/- 37)
test bench_map_std_f64           ... bench:       1,925 ns/iter (+/- 45)
test bench_map_std_u16           ... bench:       2,357 ns/iter (+/- 23)
test bench_map_std_u32           ... bench:       2,262 ns/iter (+/- 102)
test bench_map_std_u64           ... bench:       1,933 ns/iter (+/- 48)
test bench_map_std_u8            ... bench:       2,545 ns/iter (+/- 65)
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
