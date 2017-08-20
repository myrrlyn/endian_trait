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

implendian!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
	use super::*;

	#[repr(C)]
	#[derive(Clone, Debug, Endian, Eq, PartialEq)]
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
	#[derive(Clone, Debug, Endian, Eq, PartialEq)]
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

	#[test]
	fn flip_struct() {
		let eb = get_example().to_be();
		let el = get_example().to_le();
		let eb: [u8; 16] = unsafe { ::std::mem::transmute(eb) };
		let el: [u8; 16] = unsafe { ::std::mem::transmute(el) };

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

		let nb: [u8; 24] = unsafe { ::std::mem::transmute(nb) };
		let nl: [u8; 24] = unsafe { ::std::mem::transmute(nl) };

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
}
