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

/// Implement on `bool`
///
/// `bool` is always one byte, and single bytes don`t have endian order.
impl Endian for bool {
	fn from_be(self) -> Self { self }
	fn from_le(self) -> Self { self }
	fn to_be(self) -> Self { self }
	fn to_le(self) -> Self { self }
}

/// Implement on `char`
///
/// `char` is four bytes wide. Delegate to `u32`'s implementation and transmute.
/// This is safe ONLY IF THE CONVERSION MAKES LOGICAL SENSE
/// `char` is Unicode codepoints, NOT integers, so not all values of `u32` are
/// valid values of `char`.
/// The `to_` functions will emit potentially invalid `char` values, as this is
/// to be expected. The `from_` functions, however, will panic if they are about
/// to emit an invalid `char` byte value.
impl Endian for char {
	/// Attempts to create a local `char` from a big-endian value.
	///
	/// This function WILL panic if the local value exceeds the maximum Unicode
	/// Scalar Value permissible.
	fn from_be(self) -> Self {
		let flip: u32 = (self as u32).from_be();
		if flip > ::std::char::MAX as u32 {
			panic!("A `char` cannot have a value of {:X}", flip);
		}
		unsafe { ::std::mem::transmute(flip) }
	}
	/// Attempts to create a local `char` from a little-endian value.
	///
	/// This function WILL panic if the local value exceeds the maximum Unicode
	/// Scalar Value permissible.
	fn from_le(self) -> Self {
		let flip: u32 = (self as u32).from_le();
		if flip > ::std::char::MAX as u32 {
			panic!("A `char` cannot have a value of {:X}", flip);
		}
		unsafe { ::std::mem::transmute(flip) }
	}
	/// Converts a local `char` to big-endian.
	///
	/// This may result in a byte value that is not a valid Unicode Scalar Value
	/// and the result of this transform should be passed into a `from_be()`
	/// before using it in anything that requires `char` semantics.
	fn to_be(self) -> Self {
		unsafe { ::std::mem::transmute((self as u32).to_be()) }
	}
	/// Converts a local `char` to little-endian.
	///
	/// This may result in a byte value that is not a valid Unicode Scalar Value
	/// and the result of this transform should be passed into a `from_le()`
	/// before using it in anything that requires `char` semantics.
	fn to_le(self) -> Self {
		unsafe { ::std::mem::transmute((self as u32).to_le()) }
	}
}

//  Auto-implement on the numeric primitives
implendian!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

