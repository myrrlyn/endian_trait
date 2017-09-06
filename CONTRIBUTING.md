# Contributing

If you find something you want added to this, you can let me know on GitHub,
Twitter, reddit, maybe the Rust IRC, under this name, or you can file an issue
or open a PR.

I think *this* crate is done; I need help on `endian_trait_derive` though. I
want to extend the custom derivation to cover enums with `#[repr(...)]`
annotations. I will document my issues more in the child crate.

## Design Questions

I'm open to discussion on how arrays and slices should be handled, and on what
if any constraints should be placed on `Endian`. For instance, while
implementing on arrays, I found that if I could use the bounds

```rust
impl<T: Endian + Copy + !Drop> Endian for [T; N]
```

then I could use naive extraction-mutation-replacement, whereas when `T: ?Copy`
I would have to resort to unsafe, C-style pointer dereferencing to shunt each
element into local context, manipulate it, and shunt it back.

I am not entirely sure it makes sense to implement `Endian` on types that aren't
`Copy`, since this trait is only useful for copying plain data into some
serialization mechanism.

I would like to resolve whether to specify `Endian: Copy + !Drop` or leave it
unbounded before making a 1.0 release.
