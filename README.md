# Optional

[![Build Status](https://travis-ci.org/llogiq/optional.svg)](https://travis-ci.org/llogiq/optional)[![Current Version](http://meritbadge.herokuapp.com/optional)](https://crates.io/crates/optional)

This package aims to offer some `Option<_>`-like values that allow a 
space-optimized representation.

As of now there is a type `OptionBool`. It works very much like an
`Option<bool>` (and indeed mostly copies its interface). There are only a few
additional methods where `Option<bool>` would recieve or return another 
`Option<bool>`, a `_bool` prefix is used to receive/return `OptionBool` 
instead.

Then there is a struct `Optioned<T>` that is similar to Option but needs no 
additional storage, declaring a None value instead. Their are impls For 
`i8..64`, `u8..64`, `isize`, `usize`, `f32` and `f64`, and it's easy enough to 
Implement it for your own type (have a look at the Noned trait). The None value 
for the int types is their `MIN`, for the unsigned types is their `MAX` and for 
floats is `NAN`.

This crate is experimental, has almost no docs or tests. Honestly, I mainly
publish it now to embarrass myself.

# Documentation

[API documentation](https://llogiq.github.io/optional/doc/optional/index.html)
(I need to work on this)

You also may want to have a look at the infinitely better documented
[Option docs](http://doc.rust-lang.org/std/option/enum.Option.html)

# License

This project is under MIT license (see LICENSE)
