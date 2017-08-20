# Endian Trait

This library provides a trait called `Endian` that requires four methods for
converting to/from big/little endian orders.

This trait is implemented on the integer primitives `{i,u}{8,16,32,64,size}`.

Any item where an endian conversion makes sense can implement this trait, and
items composed of all `Endian` items can themselves be `Endian`

An associated crate, `endian_trait_derive`, provides a custom derive.

This will work on structs of integer primitives, and nested structs where inner
structs are themselves `Endian`.

This does ***NOT*** work on tuple structs, or on enums with specified `repr`
layouts.

I would like to expand this crate to include these types, but at the moment I am
struggling to make the derivation macro support these types.

```rust
extern crate endian_trait;
#[macro_use]
extern crate endian_trait_derive;

#[derive(Endian)]
struct Example {
    a: u64,
    b: i32,
    c: u16,
    d: u8,
}

#[derive(Endian)]
struct Nested {
    a: Example,
    b: u8,
}

#[test]
fn flip_it() {
    let e = Example {
        a: 0x0123456789abcdef,
        b: 0x01234567,
        c: 0x89ab,
        d: 0xcd,
    };

    //  I'm assuming you're on x86, a little-endian chip.
    let Example {
        a,
        b,
        c,
        d,
    } = e.to_be();

    assert_eq!(a, 0xefcdab8967452301);
    assert_eq!(b, 0x67452301);
    assert_eq!(c, 0xab89);
    assert_eq!(d, 0xcd);

    let n = Nested {
        a: Example {
            a: a,
            b: b,
            c: c,
            d: d,
        },
        b: 0xef,
    };

    assert_eq!(n.to_be().b == 0xef);
}
```

The use case for this library is for assisting with flipping complex data
structures when they move across the network boundary. Here's an example
demonstrating the (relative) ease of moving a (relatively) complex structure
across a network boundary.

```rust
extern crate endian_trait;
#[macro_use]
extern crate endian_trait_derive;

use std::convert::{From,To};

#[derive(Endian)]
struct ComplexData {
    a: ChildStruct,
    b: ChildStruct,
}
#[derive(Endian)]
struct ChildStruct {
    a: u32,
    b: u16,
}

impl From<[u8; 12]> for ComplexData {
    fn from(src: [u8; 12]) -> Self {
        use std::mem::transmute;
        unsafe {
            ComplexData {
                a: ChildStruct {
                    a: transmute(src[.. 4]),
                    b: transmute(src[4 .. 6]),
                },
                b: ChildStruct {
                    a: transmute(src[6 .. 10]),
                    b: transmute(src[10 ..]),
                },
            }.from_be()
        }
    }
}
impl To<[u8; 12]> for ComplexData {
    fn to(self) -> [u8; 12] {
        use std::mem::transmute;
        unsafe {
            let s = self.to_be();
            let out: [u8; 12];
            out[.. 4] = transmute(self.a.a);
            out[4 .. 6] = transmute(self.a.b);
            out[6 .. 10] = transmute(self.b.a);
            out[10 ..] = transmute(self.b.b);
        }
    }
}
```

Now the conversion methods that switch between the structure and a byte
representation automatically perform endian conversion as well. You may want to
keep endianness separate, and only do so at the network (so that you can
serialize to bytes for non-network transmission, for example), in which case you
would remove the Endian trait calls from the conversion traits, and do so at the
appropriate call site, like so:

```rust
let s = ComplexData { /* ... */ };
let outbound: [u8; 12] = s.to_be().into();
let inbound: [u8; 12] = read_from_network();
let r: ComplexData = inbound.into().from_be();
```
