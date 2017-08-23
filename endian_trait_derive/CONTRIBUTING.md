# Contributing

This is the first procedural macro crate I've ever written, and I presume it
shows. I need to extend the `impl_endian` function's match stack to not panic on
`#[repr(...)]`'d enums and tuple structs.

I don't know what to do about unit structs. As they are zero-sized-types, they
have no bitwise representation, so my first instinct is to panic. However, a
more robust and correct behavior might be to just skip it and not emit any code
at all?

Once I get the two other struct types done though I think this can go 1.0.

I presume serde will have good example code on which I can draw.

If you would like to contribute an implementation of the derive for enums or
tuple structs, file a PR; you can also reach me here, Twitter, reddit, or maybe
the Rust IRC, all under this name, for anything else.

Thanks!
