# Endian Trait Derivation

This provides a custom derive for the Endian trait on structs.

It currently only works on structs. Enums are not yet supported; I plan to look
into this before a 1.0 release.

This crate is only useful as a sidekick to `endian_trait`.

```rust
extern crate endian_trait;
#[macro_use]
extern crate endian_trait_derive;

#[derive(Endian)]
struct Example<A: Endian> {
    a: A,
    //  others
}
```
