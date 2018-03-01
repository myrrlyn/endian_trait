# Endian Trait Derivation

[![Crate][crate_svg]][crate]
[![Docs][docs_svg]][docs]
[![Gitlab CI Status][gitlab_svg]][gitlab]
[![Travis CI Status][travis_svg]][travis]

This provides a custom derive for the Endian trait on structs. It can operate on
normal braced structs, tuple structs, and unit structs, as well as enums with an
integer representation and no data.

Rust currently does not permit procedural macro tags to be placed on unions, so
this macro cannot support them.

This crate cannot be used standalone, as it generates code referring to the
`Endian` trait, which only exists in the `endian_trait` crate. That crate
re-exports the procedural macro defined here.

```toml
[dependencies]
endian_trait = "0.5"
```

```rust
#[macro_use]
extern crate endian_trait;
```

[crate]: https://crates.io/crates/endian_trait_derive
[crate_svg]: https://img.shields.io/crates/v/endian_trait_derive.svg
[docs]: https://docs.rs/endian_trait_derive
[docs_svg]: https://docs.rs/endian_trait_derive/badge.svg
[gitlab]: https://gitlab.com/myrrlyn/endian_trait
[gitlab_svg]: https://gitlab.com/myrrlyn/endian_trait/badges/master/build.svg
[travis]: https://travis-ci.org/myrrlyn/endian_trait
[travis_svg]: https://travis-ci.org/myrrlyn/endian_trait.svg?branch=master
