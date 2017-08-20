# Endian Trait Derivation

This provides a custom derive for the Endian trait on structs.

It currently only works on normal structs with named fields. It does not work on
tuple structs or enums. I would like to expand it to reach these before I
consider this ready for a 1.0 release.

```rust
extern crate endian_trait;
#[macro_use]
extern crate endian_trait_derive;

#[derive(Endian)]
struct Example {
    a: i32,
    //  others
}
```
