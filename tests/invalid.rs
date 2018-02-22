/*! Invalid Code Generation

The macro should successfully emit Rust code that has the correct shape, but
which will then fail to compile.

When the rest of this code is uncommented, compilation must fail. When it is
disabled, compilation must succeed.
!*/

/*
#![compile_fail]

extern crate endian_trait;
#[macro_use]
extern crate endian_trait_derive;

use endian_trait::Endian;

struct Foo {
	a: i32
}

#[derive(Endian)]
struct Bar {
	a: Foo,
	b: i32
}
*/
