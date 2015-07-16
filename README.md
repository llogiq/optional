# Optional

This package aims to offer some `Option<_>`-like values that allow a 
space-optimized representation.

As of now there is only one type: `OptionBool`. It works very much like an
`Option<bool>` (and indeed mostly copies its interface). There are only a few
additional methods where `Option<bool>` would recieve or return another 
`Option<bool>`, a `_bool` prefix is used to receive/return `OptionBool` 
instead.

This crate is experimental, has no docs and almost no tests. Honestly, I mainly
publish it now to embarrass myself.

# License

This project is under MIT license (see LICENSE)
