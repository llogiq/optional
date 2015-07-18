# Optional

This package aims to offer some `Option<_>`-like values that allow a 
space-optimized representation.

As of now there is a type `OptionBool`. It works very much like an
`Option<bool>` (and indeed mostly copies its interface). There are only a few
additional methods where `Option<bool>` would recieve or return another 
`Option<bool>`, a `_bool` prefix is used to receive/return `OptionBool` 
instead.

Then we have a struct `Optioned<T> that is similar to Option but needs 
no additional storage, declaring a None value instead. Their are impls
For i8..64, u8..64, isize., usize, f32 and f64, and it's easy enough to 
Implement it for your own type (have a look at the Noned trait).

This crate is experimental, has no docs and almost no tests. Honestly, I mainly
publish it now to embarrass myself.

# License

This project is under MIT license (see LICENSE)
