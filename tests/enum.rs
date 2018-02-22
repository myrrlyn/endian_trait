/*! Test deriving Endian on an enum

This won't compile, because Endian on an enum doesn't make any sense. Cast it to
an integer repr outside the Endian operations.

If this code is uncommented, it should emit an error message about enums not
being supported. Proc-macro panics are compile-time, not runtime, so this test
cannot be run directly.
!*/

/*
extern crate endian_trait;
#[macro_use]
extern crate endian_trait_derive;

use endian_trait::Endian;

#[test]
fn enums() {
	#[repr(u8)]
	#[derive(Endian)]
	enum Foo {
		A,
		B,
		C,
	}
}
*/
