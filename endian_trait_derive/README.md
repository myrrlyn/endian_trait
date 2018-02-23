# Endian Trait Derivation

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
