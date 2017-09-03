/*!
!*/

#[allow(unused_imports)]
#[macro_use]
extern crate endian_trait_derive;

pub trait Endian {
	fn to_be(self) -> Self;
	fn to_le(self) -> Self;
	fn from_be(self) -> Self;
	fn from_le(self) -> Self;
}

macro_rules! implendian {
	( $( $t:tt ),* ) => { $(
impl Endian for $t {
	fn to_be(self) -> Self {
		$t::to_be(self)
	}
	fn to_le(self) -> Self {
		$t::to_le(self)
	}
	fn from_be(self) -> Self {
		$t::from_be(self)
	}
	fn from_le(self) -> Self {
		$t::from_le(self)
	}
}
	)* };
}

//  Auto-implement on the numeric primitives
implendian!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
	use super::*;
	use std::mem::{
		size_of,
		transmute,
	};

	#[repr(C)]
	#[derive(Clone, Copy, Debug, Endian, Eq, PartialEq)]
	struct Example {
		a: u16,
		b: u16,
		c: u32,
		d: u64,
	}

	fn get_example() -> Example {
		Example {
			a: 0x1234,
			b: 0x5678,
			c: 0x9ABCDEF0,
			d: 0x123456789ABCDEF0,
		}
	}

	#[repr(C)]
	#[derive(Clone, Copy, Debug, Endian, Eq, PartialEq)]
	struct Nested {
		a: u64,
		b: Example,
	}

	fn get_nested() -> Nested {
		Nested {
			a: 0xabad1dea0c05fefe,
			b: get_example(),
		}
	}

	//  Time to get freaky
	#[repr(packed)]
	#[derive(Clone, Copy, Debug, Endian, Eq, PartialEq)]
	struct Tuple(u64, i32, u16, i8);

	fn get_tuple() -> Tuple {
		Tuple(
			0x0123456789ABCDEF,
			0x02468ACD,
			0x1337,
			0x5A,
		)
	}

	//  Time to get less freaky
	//  This will work on non-repr-{C,packed} types just fine; however accessing
	//  those as byte arrays is UB.
	//
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
	fn flip_struct() {
		let eb = get_example().to_be();
		let el = get_example().to_le();
		assert_eq!(size_of::<Example>(), 16);
		let eb: [u8; 16] = unsafe { transmute(eb) };
		let el: [u8; 16] = unsafe { transmute(el) };

		//  .a
		assert_eq!(eb[0], el[1]);
		assert_eq!(eb[1], el[0]);

		//  .b
		assert_eq!(eb[2], el[3]);
		assert_eq!(eb[3], el[2]);

		//  .c
		assert_eq!(eb[4], el[7]);
		assert_eq!(eb[5], el[6]);
		assert_eq!(eb[6], el[5]);
		assert_eq!(eb[7], el[4]);

		//  .d
		assert_eq!(eb[8], el[15]);
		assert_eq!(eb[9], el[14]);
		assert_eq!(eb[10], el[13]);
		assert_eq!(eb[11], el[12]);
		assert_eq!(eb[12], el[11]);
		assert_eq!(eb[13], el[10]);
		assert_eq!(eb[14], el[9]);
		assert_eq!(eb[15], el[8]);
	}

	#[test]
	fn flip_nested_struct() {
		let nb = get_nested().to_be();
		let nl = get_nested().to_le();

		let nb: [u8; 24] = unsafe { transmute(nb) };
		let nl: [u8; 24] = unsafe { transmute(nl) };

		//  .a
		assert_eq!(nb[0], nl[7]);
		assert_eq!(nb[1], nl[6]);
		assert_eq!(nb[2], nl[5]);
		assert_eq!(nb[3], nl[4]);
		assert_eq!(nb[4], nl[3]);
		assert_eq!(nb[5], nl[2]);
		assert_eq!(nb[6], nl[1]);
		assert_eq!(nb[7], nl[0]);

		//  .b.a
		assert_eq!(nb[8], nl[9]);
		assert_eq!(nb[9], nl[8]);

		//  .b.b
		assert_eq!(nb[10], nl[11]);
		assert_eq!(nb[11], nl[10]);

		//  .b.b
		assert_eq!(nb[12], nl[15]);
		assert_eq!(nb[13], nl[14]);
		assert_eq!(nb[14], nl[13]);
		assert_eq!(nb[15], nl[12]);

		//  .b.b
		assert_eq!(nb[16], nl[23]);
		assert_eq!(nb[17], nl[22]);
		assert_eq!(nb[18], nl[21]);
		assert_eq!(nb[19], nl[20]);
		assert_eq!(nb[20], nl[19]);
		assert_eq!(nb[21], nl[18]);
		assert_eq!(nb[22], nl[17]);
		assert_eq!(nb[23], nl[16]);
	}

	#[test]
	fn double_flip() {
		let e = get_example().to_be().from_be();
		assert_eq!(e, get_example());

		let e = get_example().to_le().from_le();
		assert_eq!(e, get_example());

		//  On BE chips, to_be() goes away; on LE chips, to_le() goes away.
		//  We want to test that flipping twice cancels out, so these four calls
		//  should just flip, then flip again, back to the original.
		let e = get_example().to_be().to_be().to_le().to_le();
		assert_eq!(e, get_example());
	}

	#[test]
	fn flip_tuple() {
		let tb = get_tuple().to_be();
		let tl = get_tuple().to_le();
		assert_eq!(size_of::<Tuple>(), 15);
		let tb: [u8; 15] = unsafe { transmute(tb) };
		let tl: [u8; 15] = unsafe { transmute(tl) };

		//  .0
		assert_eq!(tb[0], tl[7]);
		assert_eq!(tb[1], tl[6]);
		assert_eq!(tb[2], tl[5]);
		assert_eq!(tb[3], tl[4]);
		assert_eq!(tb[4], tl[3]);
		assert_eq!(tb[5], tl[2]);
		assert_eq!(tb[6], tl[1]);
		assert_eq!(tb[7], tl[0]);

		//  .1
		assert_eq!(tb[8], tl[11]);
		assert_eq!(tb[9], tl[10]);
		assert_eq!(tb[10], tl[9]);
		assert_eq!(tb[11], tl[8]);

		//  .2
		assert_eq!(tb[12], tl[13]);
		assert_eq!(tb[13], tl[12]);

		//  .3
		//  This should be idempotent, since a single byte has no endian
		assert_eq!(tb[14], tl[14]);
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

	//  Let's make an abomination unto the Lord and compiler
	//  It doesn't make any sense from a mechanical perspective to define Endian
	//  on a zero sized type because zero sized types don't ever exist.
	//  However, it is permissible to define all kinds of things that look at or
	//  use the byte repr on ZSTs because, since ZSTs **have no byte repr**, we
	//  just use it as abstract logic. ZSTs are all logically Eq since they are
	//  all non-existent in the same way. ZSTs can also claim to implement byte
	//  reorder methods, and the compiler will just erase them all during
	//  monomorphization.
	//
	//  ¯\_(ツ)_/¯ 🤷

	#[derive(Clone, Copy, Debug, Endian, PartialEq, Eq)]
	struct Zst;

	#[test]
	fn zst() {
		let z: Zst = Zst;
		let z2: Zst = z.to_be();
		assert_eq!(size_of::<Zst>(), 0);
		assert_eq!(z, z2);
	}
}
