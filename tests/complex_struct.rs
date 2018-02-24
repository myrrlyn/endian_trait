/*! Test Endian derivation on a struct with a non-contiguous memory repr

Endian will work on structs with repr other than C or packed just fine; it is
merely UB to transmute them into byte arrays and access into them like that,
which is what all the other tests are doing.
!*/

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;

//  This is the least possibly aligned struct I can imagine, so it should be
//  of enormous size with lots of padding.
#[derive(Clone, Copy, Debug, Endian, Eq, PartialEq)]
struct NotC {
	a: u8,
	b: u16,
	c: u8,
	d: u32,
	e: u8,
	f: u64,
}

fn get_notc() -> NotC {
	NotC {
		a: 0xA5,
		b: 0x1234,
		c: 0x5A,
		d: 0xdeadbeef,
		e: 0x42,
		f: 0xc001c0deabad1dea,
	}
}

#[test]
fn flip_notc() {
	let cb = get_notc().to_be();
	let cl = get_notc().to_le();

	assert_eq!(cb.a.from_be(), cl.a.from_le());
	assert_eq!(cb.b.from_be(), cl.b.from_le());
	assert_eq!(cb.c.from_be(), cl.c.from_le());
	assert_eq!(cb.d.from_be(), cl.d.from_le());
	assert_eq!(cb.e.from_be(), cl.e.from_le());
	assert_eq!(cb.f.from_be(), cl.f.from_le());

	assert_eq!(cb.from_be(), cl.from_le());
	assert_eq!(cb.from_be(), get_notc());
	assert_eq!(cl.from_le(), get_notc());
}
