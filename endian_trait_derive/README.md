# Endian Trait Derivation

[![Crate][crate_svg]][crate]
[![Gitlab CI Status][gitlab_svg]][gitlab]
[![Travis CI Status][travis_svg]][travis]

This provides a custom derive for the Endian trait on structs. It can operate on
normal braced structs, tuple structs, and unit structs.

It currently only works on structs. Enums and unions are not yet supported; I
plan to look into this before a 1.0 release.

This crate cannot be used standalone. Its derive macro is re-exported by the
`endian_trait` crate.

```toml
[dependencies]
endian_trait = "0.3"
```

```rust
#[macro_use]
extern crate endian_trait;
```

[crate]: https://crates.io/crates/endian_trait_derive
[crate_svg]: https://img.shields.io/crates/v/endian_trait_derive.svg
[gitlab]: https://gitlab.com/myrrlyn/endian_trait
[gitlab_svg]: https://gitlab.com/myrrlyn/endian_trait/badges/master/build.svg
[travis]: https://travis-ci.org/myrrlyn/endian_trait
[travis_svg]: https://travis-ci.org/myrrlyn/endian_trait.svg?branch=master
